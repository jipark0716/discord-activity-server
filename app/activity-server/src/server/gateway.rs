use std::pin::Pin;
use std::sync::Arc;
use crate::config::Config;
use crate::server::grpc::{event, AuthRequest, Event, MockRequest, OnJoinEvent};
use crate::server::grpc::gateway_service_server::{GatewayService};
use discord_auth::{DiscordClient, DiscordUserResponse};
use room::{RoomManager};
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::Stream;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

pub(crate) struct GatewayGrpcService {
    discord_client: DiscordClient,
    room_manager: Arc<RoomManager<GrpcSession, Event>>,
}

type EventStream = Pin<Box<dyn Stream<Item = Result<Event, Status>> + Send + 'static>>;

impl GatewayGrpcService {
    pub(crate) fn new(
        config: &Config,
        room_manager: Arc<RoomManager<GrpcSession, Event>>,
    ) -> anyhow::Result<Self> {
        let client = DiscordClient::new(
            config.discord_client_id.clone(),
            config.discord_secret_key.clone(),
        )?;

        Ok(Self {
            discord_client: client,
            room_manager,
        })
    }

    async fn create_stream(&self, instance_id: u64, user: DiscordUserResponse) -> Result<Response<EventStream>, Status> {
        let room = self.room_manager.get_or_create(instance_id);

        room.join(
            user.id,
            GrpcSession {
                user: user.clone(),
                session_id: 0,
            },
        );

        let stream = BroadcastStream::new(room.subscribe())
          .filter_map(|result| match result {
              Ok(event) => Some(Ok(event)),
              Err(_) => None,
          });

        room.broadcast(Event {
            payload: Some(event::Payload::Join(OnJoinEvent {
                event_id: room.next_event_id(),
                user_id: user.id,
                username: user.username,
                avatar: user.avatar,
            })),
        })
          .map_err(|e| {
              Status::unavailable(format!("Failed to broadcast event: {}", e))
          })?;

        Ok(Response::new(Box::pin(stream)))
    }
}

#[tonic::async_trait]
impl GatewayService for GatewayGrpcService {
    type AuthStream = EventStream;
    async fn auth(&self, request: Request<AuthRequest>) -> Result<Response<Self::AuthStream>, Status> {
        let request = request.into_inner();

        let token_response = self
            .discord_client
            .authorization(&request.code)
            .await
            .map_err(|_| Status::unavailable("fail to get discord token"))?;

        tracing::info!("{:?}", token_response);

        let user_response = self
            .discord_client
            .get_current_user(&token_response.access_token)
            .await
            .map_err(|_| Status::unavailable("fail to get discord user"))?;


        self.create_stream(request.guild_id, user_response).await
    }

    type MockStream = EventStream;
    async fn mock(&self, request: Request<MockRequest>) -> Result<Response<Self::MockStream>, Status> {
        let request = request.into_inner();

        self.create_stream(request.instance_id, DiscordUserResponse {
            id: request.id,
            username: request.username,
            avatar: request.avatar,
        }).await
    }
}

#[derive(Clone)]
pub struct GrpcSession {
    pub user: DiscordUserResponse,
    pub session_id: u64,
}

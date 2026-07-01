use crate::ext::auth::AuthorizationExt;
use crate::server::gateway::GrpcSession;
use crate::server::grpc::event::Payload;
use crate::server::grpc::message_service_server::MessageService;
use crate::server::grpc::{Event, OnJoinEvent, OnPingEvent, PingRequest};
use room::RoomManager;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub(crate) struct MessageGrpcService {
    room_manager: Arc<RoomManager<GrpcSession, Event>>,
}

impl MessageGrpcService {
    pub fn new(room_manager: Arc<RoomManager<GrpcSession, Event>>) -> Self {
        Self { room_manager }
    }
}

#[tonic::async_trait]
impl MessageService for MessageGrpcService {
    async fn create_message(&self, request: Request<PingRequest>) -> Result<Response<()>, Status> {
        let auth = request.get_authorized()?;
        
        let request = request.into_inner();

        let room = self.room_manager.get(auth.instance_id)
          .map_err(|_e| Status::unavailable("Failed to get room"))?;

        room.broadcast(Event {
            payload: Some(Payload::Ping(OnPingEvent {
                event_id: room.next_event_id(),
                user_id: auth.user_id,
                position_x: request.position_x,
                position_y: request.position_y,
            })),
        })
        .map_err(|e| Status::unavailable(format!("Failed to fire OnPingEvent: {}", e)))?;

        Ok(Response::new(()))
    }
}

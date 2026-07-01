use std::sync::Arc;
use tonic::{Request, Response, Status};
use room::RoomManager;
use crate::ext::auth::AuthorizationExt;
use crate::server::gateway::{GrpcSession};
use crate::server::grpc::auth_service_server::AuthService;
use crate::server::grpc::{Event, Member, MemberCollection};

pub(crate) struct AuthGrpcService {
    room_manager: Arc<RoomManager<GrpcSession, Event>>,
}

impl AuthGrpcService {
    pub fn new(room_manager: Arc<RoomManager<GrpcSession, Event>>) -> Self {
        Self { room_manager }
    }
}

#[tonic::async_trait]
impl AuthService for AuthGrpcService {
    async fn get_members(&self, request: Request<()>) -> Result<Response<MemberCollection>, Status> {
        let auth = request.get_authorized()?;

        let room = self.room_manager.get(auth.instance_id)
          .map_err(|_| Status::unavailable("not found instance"))?;;

        let members: Vec<Member> = room.get_members()
          .iter()
          .map(|o| Member {
              user_id: o.user.id,
              username: o.user.username.clone(),
              avatar: o.user.avatar.clone(),
          })
          .collect();

        Ok(Response::from(MemberCollection {
            members,
        }))
    }
}
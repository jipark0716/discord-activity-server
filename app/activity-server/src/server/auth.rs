use tonic::{Request, Response, Status};
use discord_auth::DiscordClient;
use crate::config::Config;
use crate::server::grpc::auth_service_server::AuthService;
use crate::server::grpc::{AuthRequest, AuthResponse};

pub(crate) struct AuthGrpcService {
    discord_client: DiscordClient,
}

impl AuthGrpcService {
    pub(crate) fn new(config: &Config) -> anyhow::Result<Self> {
        let client = DiscordClient::new(
            config.discord_client_id.clone(),
            config.discord_secret_key.clone(),
        )?;

        Ok(Self {
            discord_client: client,
        })
    }
}

#[tonic::async_trait]
impl AuthService for AuthGrpcService {
    async fn get_token(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let request = request.into_inner();

        let token_response = self
          .discord_client
          .authorization(&request.code)
          .await
          .map_err(|_| {
              Status::unavailable("fail to get discord token")
          })?;

        Ok(Response::new(AuthResponse {
            access_token: token_response.access_token,
            token_type: token_response.token_type,
            expires_in: token_response.expires_in,
            refresh_token: token_response.refresh_token,
            scope: token_response.scope,
        }))
    }
}

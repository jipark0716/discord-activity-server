use anyhow::Context;
use tonic::transport::Server;
use tracing::level_filters::LevelFilter;
use crate::server::auth::AuthGrpcService;
use crate::server::grpc::auth_service_server::AuthServiceServer;
use tracing_subscriber::{fmt, registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod config;
pub mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    registry()
      .with(fmt::layer())
      .with(LevelFilter::INFO)
      .init();

    let address = "[::]:8510".parse()?;
    let config = config::load_from_env().with_context(|| "Failed to load config")?;

    println!("{:?}", config);

    Server::builder()
      .add_service(AuthServiceServer::new(AuthGrpcService::new(&config)?))
      .serve(address)
      .await?;

    Ok(())
}
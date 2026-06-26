use anyhow::Context;
use tokio::signal::unix::{signal, SignalKind};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
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
      .layer(CorsLayer::permissive())
      .layer(GrpcWebLayer::new())
      .add_service(AuthServiceServer::new(AuthGrpcService::new(&config)?))
      .serve_with_shutdown(address, shutdown_signal())
      .await?;

    Ok(())
}

async fn shutdown_signal() {
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = sigint.recv() => {}
        _ = sigterm.recv() => {}
    }
}
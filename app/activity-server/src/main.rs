use std::sync::Arc;
use anyhow::Context;
use tokio::signal::unix::{signal, SignalKind};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
use tracing::level_filters::LevelFilter;
use crate::server::gateway::GatewayGrpcService;
use crate::server::grpc::gateway_service_server::GatewayServiceServer;
use tracing_subscriber::{fmt, registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use room::RoomManager;
use crate::server::auth::AuthGrpcService;
use crate::server::grpc::auth_service_server::AuthServiceServer;
use crate::server::grpc::message_service_server::MessageServiceServer;
use crate::server::message::MessageGrpcService;

mod config;
pub mod server;
pub mod ext;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    registry()
      .with(fmt::layer())
      .with(LevelFilter::INFO)
      .init();

    let address = "[::]:8510".parse()?;
    let config = config::load_from_env().with_context(|| "Failed to load config")?;
    let grpc_room_manager = Arc::new(RoomManager::new());

    println!("start with config: {:?}", config);

    Server::builder()
      .accept_http1(true)
      .layer(CorsLayer::permissive())
      .layer(GrpcWebLayer::new())
      .add_service(GatewayServiceServer::new(GatewayGrpcService::new(&config, grpc_room_manager.clone())?))
      .add_service(AuthServiceServer::new(AuthGrpcService::new(grpc_room_manager.clone())))
      .add_service(MessageServiceServer::new(MessageGrpcService::new(grpc_room_manager.clone())))
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
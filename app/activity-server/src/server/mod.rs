pub mod gateway;
pub mod auth;
pub mod message;

pub mod grpc {
    tonic::include_proto!("app.v1");
}
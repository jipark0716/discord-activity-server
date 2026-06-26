pub mod auth;

pub mod grpc {
    tonic::include_proto!("app.v1");
}
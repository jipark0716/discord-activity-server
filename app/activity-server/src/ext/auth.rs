use tonic::{Request, Status};

pub struct Authorization {
    pub instance_id: u64,
    pub user_id: u64,
}

pub trait AuthorizationExt {
    fn get_authorized(&self) -> Result<Authorization, Status>;
}

impl <T> AuthorizationExt for Request<T> {
    fn get_authorized(&self) -> Result<Authorization, Status> {
        let meta = self.metadata();

        let instance_id = meta
          .get("x-instance-id")
          .and_then(|value| value.to_str().ok())
          .and_then(|value| value.parse::<u64>().ok())
          .ok_or_else(|| Status::unauthenticated("invalid x-instance-id"))?;

        let user_id = meta
          .get("x-user-id")
          .and_then(|value| value.to_str().ok())
          .and_then(|value| value.parse::<u64>().ok())
          .ok_or_else(|| Status::unauthenticated("invalid x-user-id"))?;

        Ok(Authorization {
            instance_id,
            user_id,
        })
    }
}
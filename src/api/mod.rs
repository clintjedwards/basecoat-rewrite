use crate::proto::basecoat_server::Basecoat;
use crate::proto::GetSystemInfoRequest;
use crate::proto::GetSystemInfoResponse;
use tonic::{Request, Response, Status};

pub struct API {}

#[tonic::async_trait]
impl Basecoat for API {
    async fn get_system_info(
        &self,
        request: Request<GetSystemInfoRequest>,
    ) -> Result<Response<GetSystemInfoResponse>, Status> {
        Ok(Response::new(GetSystemInfoResponse {
            build_time: "".to_string(),
            commit: "".to_string(),
            debug_enabled: false,
            frontend_enabled: false,
            semver: "".to_string(),
        }))
    }
}

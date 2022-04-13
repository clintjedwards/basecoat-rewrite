use crate::proto::basecoat_server::{Basecoat, BasecoatServer};
use crate::proto::{GetSystemInfoRequest, GetSystemInfoResponse};
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder;

const BUILD_SEMVER: &str = env!("BUILD_SEMVER");
const BUILD_COMMIT: &str = env!("BUILD_COMMIT");

#[derive(Default)]
pub struct Api {}

#[tonic::async_trait]
impl Basecoat for Api {
    async fn get_system_info(
        &self,
        _: Request<GetSystemInfoRequest>,
    ) -> Result<Response<GetSystemInfoResponse>, Status> {
        Ok(Response::new(GetSystemInfoResponse {
            commit: BUILD_COMMIT.to_string(),
            debug_enabled: false,
            frontend_enabled: false,
            semver: BUILD_SEMVER.to_string(),
        }))
    }
}

// start blocking grpc server.
pub fn start(addr: &str) -> impl std::future::Future {
    let api = Api::default();

    const REFLECTION_SERVICE_DESCRIPTOR: &[u8] = tonic::include_file_descriptor_set!("reflection");

    let reflection = Builder::configure()
        .register_encoded_file_descriptor_set(REFLECTION_SERVICE_DESCRIPTOR)
        .build()
        .expect("could not build reflection server");

    Server::builder()
        .add_service(reflection)
        .add_service(BasecoatServer::new(api))
        .serve(addr.parse().unwrap())
}

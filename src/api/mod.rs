use crate::conf;
use crate::models::Organization;
use crate::proto::basecoat_server::{Basecoat, BasecoatServer};
use crate::proto::{
    CreateOrganizationRequest, CreateOrganizationResponse, GetSystemInfoRequest,
    GetSystemInfoResponse,
};
use crate::storage;
use slog_scope::info;
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder;

const BUILD_SEMVER: &str = env!("BUILD_SEMVER");
const BUILD_COMMIT: &str = env!("BUILD_COMMIT");

#[derive(Default, Clone)]
pub struct Api {
    conf: conf::api::Config,
    storage: storage::Db,
}

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

    async fn create_organization(
        &self,
        request: Request<CreateOrganizationRequest>,
    ) -> Result<Response<CreateOrganizationResponse>, Status> {
        let org = Organization::new(&request.into_inner().name);
        self.storage.create_organization(&org).await;

        info!("Created new organization"; "org" => format!("{:?}",org));
        Ok(Response::new(CreateOrganizationResponse {}))
    }
}

impl Api {
    pub async fn new(conf: conf::api::Config) -> Self {
        let storage = storage::Db::new(&conf.server.storage_path).await.unwrap();

        Api { conf, storage }
    }

    // Start blocking grpc server.
    pub async fn start(&self) {
        let reflection = Builder::configure()
            .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!("reflection"))
            .build()
            .expect("could not build reflection server");

        info!("started grpc service"; "address" => &self.conf.server.url);

        Server::builder()
            .add_service(reflection)
            .add_service(BasecoatServer::new(self.clone()))
            .serve(self.conf.server.url.parse().unwrap())
            .await
            .unwrap();
    }
}

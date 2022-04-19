use crate::conf;
use crate::models::{Organization, User};
use crate::proto;
use crate::proto::basecoat_server::{Basecoat, BasecoatServer};
use crate::proto::*;
use crate::storage;
use bcrypt::{hash, DEFAULT_COST};
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

    async fn list_organizations(
        &self,
        _: Request<ListOrganizationsRequest>,
    ) -> Result<Response<ListOrganizationsResponse>, Status> {
        let orgs_raw = self.storage.list_organizations().await;
        let orgs = orgs_raw
            .into_iter()
            .map(proto::Organization::from)
            .collect();

        Ok(Response::new(ListOrganizationsResponse {
            organizations: orgs,
        }))
    }

    async fn describe_organization(
        &self,
        request: Request<DescribeOrganizationRequest>,
    ) -> Result<Response<DescribeOrganizationResponse>, Status> {
        let org: proto::Organization = self
            .storage
            .get_organization(&request.into_inner().id)
            .await
            .into();

        Ok(Response::new(DescribeOrganizationResponse {
            organization: Some(org),
        }))
    }

    async fn list_users(
        &self,
        request: Request<ListUsersRequest>,
    ) -> Result<Response<ListUsersResponse>, Status> {
        let users_raw = self.storage.list_users(&request.into_inner().org_id).await;
        let users = users_raw.into_iter().map(proto::User::from).collect();

        Ok(Response::new(ListUsersResponse { users }))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let args = &request.into_inner();

        let user = User::new(&args.org_id, &args.name, &args.password);

        self.storage.create_user(&user).await;

        info!("Created new user"; "id" => user.id, "name" => user.name);
        Ok(Response::new(CreateUserResponse {}))
    }

    async fn describe_user(
        &self,
        request: Request<DescribeUserRequest>,
    ) -> Result<Response<DescribeUserResponse>, Status> {
        let args = &request.into_inner();

        let user: proto::User = self.storage.get_user(&args.org_id, &args.id).await.into();

        Ok(Response::new(DescribeUserResponse { user: Some(user) }))
    }

    async fn reset_user_password(
        &self,
        request: Request<ResetUserPasswordRequest>,
    ) -> Result<Response<ResetUserPasswordResponse>, Status> {
        let args = &request.into_inner();
        let hashed = hash(&args.password, DEFAULT_COST).unwrap();

        self.storage
            .reset_user_password(&args.org_id, &args.id, &hashed)
            .await;

        info!("Password reset for user"; "org" => &args.org_id, "id" => &args.id);
        Ok(Response::new(ResetUserPasswordResponse {}))
    }

    async fn toggle_user_state(
        &self,
        request: Request<ToggleUserStateRequest>,
    ) -> Result<Response<ToggleUserStateResponse>, Status> {
        let args = &request.into_inner();

        //TODO(clintjedwards): toggle_user_state should return the new state.
        self.storage.toggle_user_state(&args.org_id, &args.id).await;

        info!("User state toggled"; "org" => &args.org_id, "id" => &args.id);
        Ok(Response::new(ToggleUserStateResponse {}))
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

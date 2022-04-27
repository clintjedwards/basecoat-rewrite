use crate::conf;
use crate::models::{Base, Colorant, Contractor, Formula, Job, Organization, User};
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

    async fn list_formulas(
        &self,
        request: Request<ListFormulasRequest>,
    ) -> Result<Response<ListFormulasResponse>, Status> {
        let formulas_raw = self
            .storage
            .list_formulas(&request.into_inner().org_id)
            .await;
        let formulas = formulas_raw.into_iter().map(proto::Formula::from).collect();

        Ok(Response::new(ListFormulasResponse { formulas }))
    }

    async fn create_formula(
        &self,
        request: Request<CreateFormulaRequest>,
    ) -> Result<Response<CreateFormulaResponse>, Status> {
        let args = &request.into_inner();

        let new_formula: Formula = args.to_owned().into();

        self.storage.create_formula(&new_formula).await;

        info!("Created new formula"; "formula" => format!("{:?}",new_formula));
        Ok(Response::new(CreateFormulaResponse {}))
    }

    async fn update_formula(
        &self,
        request: Request<UpdateFormulaRequest>,
    ) -> Result<Response<UpdateFormulaResponse>, Status> {
        let args = &request.into_inner();

        let new_formula: Formula = args.to_owned().into();

        self.storage.update_formula(&new_formula).await;

        info!("Updated formula"; "formula" => format!("{:?}",new_formula));
        Ok(Response::new(UpdateFormulaResponse {}))
    }

    async fn describe_formula(
        &self,
        request: Request<DescribeFormulaRequest>,
    ) -> Result<Response<DescribeFormulaResponse>, Status> {
        let args = &request.into_inner();

        let formula: proto::Formula = self
            .storage
            .get_formula(&args.org_id, &args.id)
            .await
            .into();

        Ok(Response::new(DescribeFormulaResponse {
            formula: Some(formula),
        }))
    }

    async fn delete_formula(
        &self,
        request: Request<DeleteFormulaRequest>,
    ) -> Result<Response<DeleteFormulaResponse>, Status> {
        let args = &request.into_inner();

        self.storage.delete_formula(&args.org_id, &args.id).await;

        Ok(Response::new(DeleteFormulaResponse {}))
    }

    async fn list_colorants(
        &self,
        request: Request<ListColorantsRequest>,
    ) -> Result<Response<ListColorantsResponse>, Status> {
        let colorants_raw = self
            .storage
            .list_colorants(&request.into_inner().org_id)
            .await;
        let colorants = colorants_raw
            .into_iter()
            .map(proto::Colorant::from)
            .collect();

        Ok(Response::new(ListColorantsResponse { colorants }))
    }

    async fn create_colorant(
        &self,
        request: Request<CreateColorantRequest>,
    ) -> Result<Response<CreateColorantResponse>, Status> {
        let args = &request.into_inner();

        let colorant = Colorant::new(&args.org_id, &args.name, Some(args.manufacturer.clone()));

        self.storage.create_colorant(&colorant).await;

        info!("Created new colorant"; "colorant" => format!("{:?}",colorant));
        Ok(Response::new(CreateColorantResponse {}))
    }

    async fn describe_colorant(
        &self,
        request: Request<DescribeColorantRequest>,
    ) -> Result<Response<DescribeColorantResponse>, Status> {
        let args = &request.into_inner();

        let colorant: proto::Colorant = self
            .storage
            .get_colorant(&args.org_id, &args.id)
            .await
            .into();

        Ok(Response::new(DescribeColorantResponse {
            colorant: Some(colorant),
        }))
    }

    async fn delete_colorant(
        &self,
        request: Request<DeleteColorantRequest>,
    ) -> Result<Response<DeleteColorantResponse>, Status> {
        let args = &request.into_inner();

        self.storage.delete_colorant(&args.org_id, &args.id).await;

        Ok(Response::new(DeleteColorantResponse {}))
    }

    async fn list_bases(
        &self,
        request: Request<ListBasesRequest>,
    ) -> Result<Response<ListBasesResponse>, Status> {
        let bases_raw = self.storage.list_bases(&request.into_inner().org_id).await;
        let bases = bases_raw.into_iter().map(proto::Base::from).collect();

        Ok(Response::new(ListBasesResponse { bases }))
    }

    async fn create_base(
        &self,
        request: Request<CreateBaseRequest>,
    ) -> Result<Response<CreateBaseResponse>, Status> {
        let args = &request.into_inner();

        let base = Base::new(&args.org_id, &args.name, Some(args.manufacturer.clone()));

        self.storage.create_base(&base).await;

        info!("Created new base"; "base" => format!("{:?}",base));
        Ok(Response::new(CreateBaseResponse {}))
    }

    async fn describe_base(
        &self,
        request: Request<DescribeBaseRequest>,
    ) -> Result<Response<DescribeBaseResponse>, Status> {
        let args = &request.into_inner();

        let base: proto::Base = self.storage.get_base(&args.org_id, &args.id).await.into();

        Ok(Response::new(DescribeBaseResponse { base: Some(base) }))
    }

    async fn delete_base(
        &self,
        request: Request<DeleteBaseRequest>,
    ) -> Result<Response<DeleteBaseResponse>, Status> {
        let args = &request.into_inner();

        self.storage.delete_base(&args.org_id, &args.id).await;

        Ok(Response::new(DeleteBaseResponse {}))
    }

    async fn list_contractors(
        &self,
        request: Request<ListContractorsRequest>,
    ) -> Result<Response<ListContractorsResponse>, Status> {
        let contractors_raw = self
            .storage
            .list_contractors(&request.into_inner().org_id)
            .await;
        let contractors = contractors_raw
            .into_iter()
            .map(proto::Contractor::from)
            .collect();

        Ok(Response::new(ListContractorsResponse { contractors }))
    }

    async fn create_contractor(
        &self,
        request: Request<CreateContractorRequest>,
    ) -> Result<Response<CreateContractorResponse>, Status> {
        let args = &request.into_inner();

        let new_contractor = Contractor::new(&args.org_id, &args.name, Some(args.contact.clone()));

        self.storage.create_contractor(&new_contractor).await;

        info!("Created new contractor"; "contractor" => format!("{:?}",new_contractor));
        Ok(Response::new(CreateContractorResponse {}))
    }

    async fn update_contractor(
        &self,
        request: Request<UpdateContractorRequest>,
    ) -> Result<Response<UpdateContractorResponse>, Status> {
        let args = &request.into_inner();

        let mut updated_contractor =
            Contractor::new(&args.org_id, &args.name, Some(args.contact.clone()));
        updated_contractor.id = args.id.clone();

        self.storage.update_contractor(&updated_contractor).await;

        info!("Updated contractor"; "contractor" => format!("{:?}",updated_contractor));
        Ok(Response::new(UpdateContractorResponse {}))
    }

    async fn describe_contractor(
        &self,
        request: Request<DescribeContractorRequest>,
    ) -> Result<Response<DescribeContractorResponse>, Status> {
        let args = &request.into_inner();

        let contractor: proto::Contractor = self
            .storage
            .get_contractor(&args.org_id, &args.id)
            .await
            .into();

        Ok(Response::new(DescribeContractorResponse {
            contractor: Some(contractor),
        }))
    }

    async fn delete_contractor(
        &self,
        request: Request<DeleteContractorRequest>,
    ) -> Result<Response<DeleteContractorResponse>, Status> {
        let args = &request.into_inner();

        self.storage.delete_contractor(&args.org_id, &args.id).await;

        Ok(Response::new(DeleteContractorResponse {}))
    }

    async fn list_jobs(
        &self,
        request: Request<ListJobsRequest>,
    ) -> Result<Response<ListJobsResponse>, Status> {
        let args = &request.into_inner();

        let jobs_raw = self
            .storage
            .list_jobs(&args.org_id, &args.contractor_id)
            .await;
        let jobs = jobs_raw.into_iter().map(proto::Job::from).collect();

        Ok(Response::new(ListJobsResponse { jobs }))
    }

    async fn create_job(
        &self,
        request: Request<CreateJobRequest>,
    ) -> Result<Response<CreateJobResponse>, Status> {
        let args = &request.into_inner();

        let new_job = Job::new(
            &args.org_id,
            &args.name,
            &args.contractor_id,
            Some(args.contact.clone()),
            Some(args.address.clone()),
            Some(args.notes.clone()),
        );

        self.storage.create_job(&new_job).await;

        info!("Created new job"; "job" => format!("{:?}",new_job));
        Ok(Response::new(CreateJobResponse {}))
    }

    async fn update_job(
        &self,
        request: Request<UpdateJobRequest>,
    ) -> Result<Response<UpdateJobResponse>, Status> {
        let args = &request.into_inner();

        let mut updated_job = Job::new(
            &args.org_id,
            &args.name,
            &args.contractor_id,
            Some(args.contact.clone()),
            Some(args.address.clone()),
            Some(args.notes.clone()),
        );
        updated_job.id = args.id.clone();

        self.storage.update_job(&updated_job).await;

        info!("Updated job"; "job" => format!("{:?}",updated_job));
        Ok(Response::new(UpdateJobResponse {}))
    }

    async fn describe_job(
        &self,
        request: Request<DescribeJobRequest>,
    ) -> Result<Response<DescribeJobResponse>, Status> {
        let args = &request.into_inner();

        let job: proto::Job = self
            .storage
            .get_job(&args.org_id, &args.id, &args.contractor_id)
            .await
            .into();

        Ok(Response::new(DescribeJobResponse { job: Some(job) }))
    }

    async fn delete_job(
        &self,
        request: Request<DeleteJobRequest>,
    ) -> Result<Response<DeleteJobResponse>, Status> {
        let args = &request.into_inner();

        self.storage
            .delete_job(&args.org_id, &args.id, &args.contractor_id)
            .await;

        Ok(Response::new(DeleteJobResponse {}))
    }

    async fn attach_formula_to_job(
        &self,
        request: Request<AttachFormulaToJobRequest>,
    ) -> Result<Response<AttachFormulaToJobResponse>, Status> {
        let args = &request.into_inner();

        self.storage
            .attach_formula_to_job(&args.org_id, &args.formula_id, &args.job_id)
            .await;

        Ok(Response::new(AttachFormulaToJobResponse {}))
    }

    async fn detach_formula_from_job(
        &self,
        request: Request<DetachFormulaFromJobRequest>,
    ) -> Result<Response<DetachFormulaFromJobResponse>, Status> {
        let args = &request.into_inner();

        self.storage
            .detach_formula_from_job(&args.org_id, &args.formula_id, &args.job_id)
            .await;

        Ok(Response::new(DetachFormulaFromJobResponse {}))
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

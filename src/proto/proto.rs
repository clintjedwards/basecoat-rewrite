/// Organization represents a group of user accounts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Organization {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub created: i64,
    #[prost(int64, tag="4")]
    pub modified: i64,
}
/// User represents a user user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="user::State", tag="2")]
    pub state: i32,
    #[prost(int64, tag="3")]
    pub created: i64,
    #[prost(int64, tag="4")]
    pub modified: i64,
    #[prost(string, tag="5")]
    pub org_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `User`.
pub mod user {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Unknown = 0,
        Active = 1,
        Disabled = 2,
    }
}
/// Service
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSystemInfoRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSystemInfoResponse {
    #[prost(string, tag="1")]
    pub commit: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub debug_enabled: bool,
    #[prost(bool, tag="3")]
    pub frontend_enabled: bool,
    #[prost(string, tag="4")]
    pub semver: ::prost::alloc::string::String,
}
/// Organization
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsResponse {
    #[prost(message, repeated, tag="1")]
    pub organizations: ::prost::alloc::vec::Vec<Organization>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrganizationRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrganizationResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeOrganizationRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeOrganizationResponse {
    #[prost(message, optional, tag="1")]
    pub organization: ::core::option::Option<Organization>,
}
/// User
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersResponse {
    #[prost(message, repeated, tag="1")]
    pub users: ::prost::alloc::vec::Vec<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeUserRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeUserResponse {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetUserPasswordRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetUserPasswordResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleUserStateRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToggleUserStateResponse {
}
/// Generated client implementations.
pub mod basecoat_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct BasecoatClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BasecoatClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BasecoatClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BasecoatClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BasecoatClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// System endpoints
        pub async fn get_system_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSystemInfoRequest>,
        ) -> Result<tonic::Response<super::GetSystemInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Basecoat/GetSystemInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Organization routes (Admin only)
        pub async fn list_organizations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrganizationsRequest>,
        ) -> Result<tonic::Response<super::ListOrganizationsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Basecoat/ListOrganizations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrganizationRequest>,
        ) -> Result<tonic::Response<super::CreateOrganizationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Basecoat/CreateOrganization",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn describe_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeOrganizationRequest>,
        ) -> Result<
                tonic::Response<super::DescribeOrganizationResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Basecoat/DescribeOrganization",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// User routes (Admin only)
        pub async fn list_users(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsersRequest>,
        ) -> Result<tonic::Response<super::ListUsersResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Basecoat/ListUsers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn describe_user(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeUserRequest>,
        ) -> Result<tonic::Response<super::DescribeUserResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Basecoat/DescribeUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_user(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserRequest>,
        ) -> Result<tonic::Response<super::CreateUserResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Basecoat/CreateUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn reset_user_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetUserPasswordRequest>,
        ) -> Result<tonic::Response<super::ResetUserPasswordResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Basecoat/ResetUserPassword",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn toggle_user_state(
            &mut self,
            request: impl tonic::IntoRequest<super::ToggleUserStateRequest>,
        ) -> Result<tonic::Response<super::ToggleUserStateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Basecoat/ToggleUserState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod basecoat_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with BasecoatServer.
    #[async_trait]
    pub trait Basecoat: Send + Sync + 'static {
        /// System endpoints
        async fn get_system_info(
            &self,
            request: tonic::Request<super::GetSystemInfoRequest>,
        ) -> Result<tonic::Response<super::GetSystemInfoResponse>, tonic::Status>;
        /// Organization routes (Admin only)
        async fn list_organizations(
            &self,
            request: tonic::Request<super::ListOrganizationsRequest>,
        ) -> Result<tonic::Response<super::ListOrganizationsResponse>, tonic::Status>;
        async fn create_organization(
            &self,
            request: tonic::Request<super::CreateOrganizationRequest>,
        ) -> Result<tonic::Response<super::CreateOrganizationResponse>, tonic::Status>;
        async fn describe_organization(
            &self,
            request: tonic::Request<super::DescribeOrganizationRequest>,
        ) -> Result<tonic::Response<super::DescribeOrganizationResponse>, tonic::Status>;
        /// User routes (Admin only)
        async fn list_users(
            &self,
            request: tonic::Request<super::ListUsersRequest>,
        ) -> Result<tonic::Response<super::ListUsersResponse>, tonic::Status>;
        async fn describe_user(
            &self,
            request: tonic::Request<super::DescribeUserRequest>,
        ) -> Result<tonic::Response<super::DescribeUserResponse>, tonic::Status>;
        async fn create_user(
            &self,
            request: tonic::Request<super::CreateUserRequest>,
        ) -> Result<tonic::Response<super::CreateUserResponse>, tonic::Status>;
        async fn reset_user_password(
            &self,
            request: tonic::Request<super::ResetUserPasswordRequest>,
        ) -> Result<tonic::Response<super::ResetUserPasswordResponse>, tonic::Status>;
        async fn toggle_user_state(
            &self,
            request: tonic::Request<super::ToggleUserStateRequest>,
        ) -> Result<tonic::Response<super::ToggleUserStateResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BasecoatServer<T: Basecoat> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Basecoat> BasecoatServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BasecoatServer<T>
    where
        T: Basecoat,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/proto.Basecoat/GetSystemInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetSystemInfoSvc<T: Basecoat>(pub Arc<T>);
                    impl<
                        T: Basecoat,
                    > tonic::server::UnaryService<super::GetSystemInfoRequest>
                    for GetSystemInfoSvc<T> {
                        type Response = super::GetSystemInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSystemInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_system_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSystemInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Basecoat/ListOrganizations" => {
                    #[allow(non_camel_case_types)]
                    struct ListOrganizationsSvc<T: Basecoat>(pub Arc<T>);
                    impl<
                        T: Basecoat,
                    > tonic::server::UnaryService<super::ListOrganizationsRequest>
                    for ListOrganizationsSvc<T> {
                        type Response = super::ListOrganizationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListOrganizationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_organizations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListOrganizationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Basecoat/CreateOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOrganizationSvc<T: Basecoat>(pub Arc<T>);
                    impl<
                        T: Basecoat,
                    > tonic::server::UnaryService<super::CreateOrganizationRequest>
                    for CreateOrganizationSvc<T> {
                        type Response = super::CreateOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_organization(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateOrganizationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Basecoat/DescribeOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct DescribeOrganizationSvc<T: Basecoat>(pub Arc<T>);
                    impl<
                        T: Basecoat,
                    > tonic::server::UnaryService<super::DescribeOrganizationRequest>
                    for DescribeOrganizationSvc<T> {
                        type Response = super::DescribeOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DescribeOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).describe_organization(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DescribeOrganizationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Basecoat/ListUsers" => {
                    #[allow(non_camel_case_types)]
                    struct ListUsersSvc<T: Basecoat>(pub Arc<T>);
                    impl<
                        T: Basecoat,
                    > tonic::server::UnaryService<super::ListUsersRequest>
                    for ListUsersSvc<T> {
                        type Response = super::ListUsersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUsersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_users(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListUsersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Basecoat/DescribeUser" => {
                    #[allow(non_camel_case_types)]
                    struct DescribeUserSvc<T: Basecoat>(pub Arc<T>);
                    impl<
                        T: Basecoat,
                    > tonic::server::UnaryService<super::DescribeUserRequest>
                    for DescribeUserSvc<T> {
                        type Response = super::DescribeUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DescribeUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).describe_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DescribeUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Basecoat/CreateUser" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserSvc<T: Basecoat>(pub Arc<T>);
                    impl<
                        T: Basecoat,
                    > tonic::server::UnaryService<super::CreateUserRequest>
                    for CreateUserSvc<T> {
                        type Response = super::CreateUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Basecoat/ResetUserPassword" => {
                    #[allow(non_camel_case_types)]
                    struct ResetUserPasswordSvc<T: Basecoat>(pub Arc<T>);
                    impl<
                        T: Basecoat,
                    > tonic::server::UnaryService<super::ResetUserPasswordRequest>
                    for ResetUserPasswordSvc<T> {
                        type Response = super::ResetUserPasswordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResetUserPasswordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).reset_user_password(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResetUserPasswordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Basecoat/ToggleUserState" => {
                    #[allow(non_camel_case_types)]
                    struct ToggleUserStateSvc<T: Basecoat>(pub Arc<T>);
                    impl<
                        T: Basecoat,
                    > tonic::server::UnaryService<super::ToggleUserStateRequest>
                    for ToggleUserStateSvc<T> {
                        type Response = super::ToggleUserStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ToggleUserStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).toggle_user_state(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ToggleUserStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Basecoat> Clone for BasecoatServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Basecoat> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Basecoat> tonic::transport::NamedService for BasecoatServer<T> {
        const NAME: &'static str = "proto.Basecoat";
    }
}

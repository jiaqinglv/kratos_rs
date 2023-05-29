use std::sync::Arc;

use axum::Router;
use tower_http::trace::TraceLayer;
use proto::hello_world_v1::greeter_server::GreeterServer;
use crate::{api, proto, service::{WebServices}};

pub fn new_http_router() -> Router {
    let router = Router::new();
    // router.route("/favicon.ico", get());
    let api_router = api::register_http();
    router.merge(api_router).layer(TraceLayer::new_for_http())
}

// 注册 GRPC 路由
pub fn new_grpc_router(server: &mut tonic::transport::server::Server, services: Arc<WebServices>) -> tonic::transport::server::Router {
    let gs: api::v1::hello::grpc::GreeterService = api::v1::hello::grpc::GreeterService::new(services);
    let gs: GreeterServer<api::v1::hello::grpc::GreeterService> = GreeterServer::new(gs);

    let router = server
        .add_service(gs);

    return  api::register_grpc(router);
}

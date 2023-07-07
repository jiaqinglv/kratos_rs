use std::sync::Arc;

use axum::Router;
use tower_http::trace::TraceLayer;
use crate::{api,  service::{WebServices}};

pub fn new_http_router() -> Router {
    let router = Router::new();
    // router.route("/favicon.ico", get());
    let api_router = api::register_http();
    router.merge(api_router).layer(TraceLayer::new_for_http())
}

// 注册 GRPC 路由
pub fn new_grpc_router(server: &mut tonic::transport::server::Server, services: Arc<WebServices>) -> tonic::transport::server::Router {
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(crate::proto::public::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    let router = server.add_service(service);

    return  api::register_grpc(router, services);
}

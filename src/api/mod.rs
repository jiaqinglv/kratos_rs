use std::sync::Arc;

use axum::Router;

use crate::service::WebServices;

pub mod hello;

// 注册HTTP路由
pub fn register_http() -> axum::Router {
    let router = Router::new();

    // api 路由
    let api = Router::new()
        .merge(hello::register_http());

    // 路由嵌套
    router.nest("/api", api)
}

// 注册 GRPC 路由
pub fn register_grpc(router: tonic::transport::server::Router, services: Arc<WebServices>) ->  tonic::transport::server::Router {

    let hr = hello::register_grpc(router, services.clone());

    return  hr;
}

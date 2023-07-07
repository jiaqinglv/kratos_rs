use std::sync::Arc;

use axum::Router;

use crate::service::WebServices;

pub mod v1;



pub fn register_http() -> axum::Router {
    // 版本路由
    let router = Router::new();

    // v1版本 路由
    let v1 = Router::new().nest("/hello", v1::register_http());

    // 路由嵌套
    router.nest("/v1", v1)
}

// 注册 GRPC 路由
pub fn register_grpc(router: tonic::transport::server::Router, services: Arc<WebServices>) ->  tonic::transport::server::Router {
    return  v1::register_grpc(router, services);
}

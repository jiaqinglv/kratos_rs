use std::sync::Arc;

use axum::{Router, routing::get};
use service::WebServices;

use protos::hello_world_v1::greeter_server::GreeterServer;

pub mod grpc;
pub mod http;
pub mod requset;
pub mod response;

pub fn register_http() -> Router {
    let r = Router::new();
    r.route("/:name", get(http::get_hello))
}


// 注册 GRPC 路由
pub fn register_grpc(router: tonic::transport::server::Router, services: Arc<WebServices>) ->  tonic::transport::server::Router {
    let gs: grpc::GreeterService = grpc::GreeterService::new(services);
    let gs: GreeterServer<grpc::GreeterService> = GreeterServer::new(gs);
    
    return  router.add_service(gs);
}


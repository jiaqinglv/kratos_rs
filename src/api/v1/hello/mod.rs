use axum::{routing::get,  Router};

pub mod grpc;
pub mod http;
pub mod requset;
pub mod response;

pub fn register_http() -> Router {
    let r = Router::new();
    r.route("/:name", get(http::get_hello))
}


// 注册 GRPC 路由
pub fn register_grpc(router: tonic::transport::server::Router) ->  tonic::transport::server::Router {
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(crate::proto::hello_world_v1::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    return  router.add_service(service);
}

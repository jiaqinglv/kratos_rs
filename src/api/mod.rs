use axum::Router;

pub mod v1;

// 注册HTTP路由
pub fn register_http() -> axum::Router {
    // v1版本
    let router = Router::new();

    // api 路由
    let v1 = Router::new().merge(v1::register_http());

    // 路由嵌套
    router.nest("/api", v1)
}

// 注册 GRPC 路由
pub fn register_grpc(router: tonic::transport::server::Router) ->  tonic::transport::server::Router {

    return  v1::register_grpc(router);
}

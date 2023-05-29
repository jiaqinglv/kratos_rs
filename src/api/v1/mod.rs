use axum::Router;
pub mod hello;

// pub fn register_http() -> Router {
//     let r = Router::new();
//     r.route("/hello/:name", get(hello::get_hello))
// }

pub fn register_http() -> axum::Router {
    // 版本路由
    let router = Router::new();

    // v1版本 路由
    let v1 = Router::new().nest("/hello", hello::register_http());

    // 路由嵌套
    router.nest("/v1", v1)
}

// 注册 GRPC 路由
pub fn register_grpc(router: tonic::transport::server::Router) ->  tonic::transport::server::Router {
    
    return  hello::register_grpc(router);
}


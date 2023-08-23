use axum::{extract::Path, response::Json,  Extension};
use hyper::StatusCode;
use service::WebServices;
use std::sync::Arc;

use crate::Response;
use super::response::HelloResponse;

#[axum::debug_handler]
pub async fn get_hello(
    Path(name): Path<String>,
    Extension(services): Extension<Arc<WebServices>>,
) -> (StatusCode, Json<Response<HelloResponse>>) {
    (StatusCode::OK, Json(Response {
        status: true, 
        code: 200, 
        message: None, 
        data: HelloResponse::from_biz(services.hello_service.create(name).await)
    }))
}

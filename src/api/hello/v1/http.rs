use axum::{extract::Path, response::Json,  Extension};
use hyper::StatusCode;
use crate::service::WebServices;
use std::sync::Arc;
use super::response;

pub async fn get_hello(
    Path(name): Path<String>,
    Extension(services): Extension<Arc<WebServices>>,
) -> (StatusCode, Json<response::HelloResponse>) {
    (StatusCode::OK, services.hello_service.create(name).await)
}

use std::sync::Arc;

use axum::{extract::Path, response::Json, Extension};
use constant;
use hyper::StatusCode;
use service::WebServices;

use super::response::HelloResponse;
use crate::Response;

#[axum::debug_handler]
pub async fn get_hello(
    Path(name): Path<String>,
    Extension(services): Extension<Arc<WebServices>>,
) -> (StatusCode, Json<Response<HelloResponse>>) {
    match services.hello_service.create(name).await {
        Ok(data) => (
            StatusCode::OK,
            Json(Response {
                status: true,
                code: 200,
                message: None,
                data: Some(HelloResponse::from_biz(data)),
            }),
        ),
        Err(err) => {
            if err.code == constant::ErrorCode::BadReuestError as i32 {
                (
                    StatusCode::BAD_REQUEST,
                    Json(Response {
                        status: true,
                        code: err.code,
                        message: Some("请求错误".to_string()),
                        data: None,
                    }),
                )
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    Json(Response {
                        status: true,
                        code: err.code,
                        message: Some("请求错误".to_string()),
                        data: None,
                    }),
                )
            }
        }
    }
}

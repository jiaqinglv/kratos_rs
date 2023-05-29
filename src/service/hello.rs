use std::future::Future;

use axum::Json;

use super::WebService;
use crate::{api::v1::hello::response, biz::hello};

#[derive(Debug, Default)]
pub struct HelloService {
    pub name: &'static str,
    uc: hello::HelloUsecase,
}

// 创建一个 Hello 服务
pub fn new_hello_service(name: &'static str, uc: hello::HelloUsecase) -> HelloService {
    return HelloService { name, uc };
}

impl WebService for HelloService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl HelloService {
    // 不对外开放
    async fn biz_to_res(hello: hello::Hello) -> response::HelloResponse {
        response::HelloResponse { name: hello.name }
    }

    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }

    // 创建
    pub async fn create(&self, name: String) -> Json<response::HelloResponse> {
        let hello = self.uc.repo.create(hello::Hello { name }).await;
        return Json(Self::biz_to_res(hello).await);
    }
}

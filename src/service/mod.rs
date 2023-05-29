pub mod hello;

use std::future::Future;

pub trait WebService {
    type NameFuture<'a>: Future<Output = &'static str> + Send + 'a
    where
        Self: 'a;
    fn get_service_name(&self) -> Self::NameFuture<'_>;
}

// web service集合
#[derive(Default)]
pub struct WebServices {
    pub hello_service: hello::HelloService,
}

// 创建 web service集合
pub fn new_web_services(hello_service: hello::HelloService) -> WebServices {
    WebServices { hello_service }
}

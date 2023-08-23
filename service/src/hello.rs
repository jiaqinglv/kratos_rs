use std::future::Future;

use biz::hello;

use super::WebService;

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
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }

    // 创建
    pub async fn create(&self, name: String) -> biz::hello::Hello{
        let hello_data = self.uc.repo.create(data::hello::Hello { name }).await;
        return biz::hello::Hello::new(hello_data.name);
    }
}

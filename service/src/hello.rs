use std::future::Future;

use biz::hello;
use constant;
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
    #[tracing::instrument]
    pub async fn create(&self, name: String) -> Result<biz::hello::Hello, kratos_core_rs::error::Error>{
        match self.uc.create(biz::hello::Hello{name}).await {
            Ok(project) => Ok(biz::hello::Hello{name: project.name}),
            Err(mut err) => {
                if err.code != 0 {
                    return  Err(err);
                } else{
                    err.code = constant::ErrorCode::BizError as i32;
                    return Err(err);
                }
            },
        }
    }
}

use crate::{
    biz::hello::new_hello_usecase,
    config::BootConfig,
    data::{hello::new_hello_repo, Data},
    service::{hello::new_hello_service, new_web_services, WebServices},
};

pub fn wire_app(_conf: &BootConfig, data: Data) -> WebServices {
    // hello
    let hello_repo = new_hello_repo(data);
    let hello_usecase = new_hello_usecase(hello_repo);
    let hello_service = new_hello_service("hello_service", hello_usecase);

    new_web_services(hello_service)
}

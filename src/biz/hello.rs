use crate::data::hello;

pub struct Hello {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct HelloUsecase {
    pub repo: hello::HelloRepo,
}

pub fn new_hello_usecase(repo: hello::HelloRepo) -> HelloUsecase {
    return HelloUsecase { repo };
}

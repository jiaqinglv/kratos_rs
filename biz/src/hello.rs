use data;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    pub name: String,
}

impl Hello {
    pub fn new(name: String) -> Hello {
        return Hello{
            name
        };
    }
}

#[derive(Debug, Default)]
pub struct HelloUsecase {
    pub repo: data::hello::HelloRepo,
}

pub fn new_hello_usecase(repo: data::hello::HelloRepo) -> HelloUsecase {
    return HelloUsecase { repo };
}

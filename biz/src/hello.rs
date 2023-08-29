use data;
use serde::{Deserialize, Serialize};
use constant;

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

impl HelloUsecase {
    #[tracing::instrument]
    pub async fn create(&self, data: Hello) -> Result<Hello,kratos_core_rs::error::Error> {
        match self.repo.create(data::hello::Hello { name: data.name }).await {
            Ok(p) => Ok(Hello{
                name:p.name
            }),
            Err(err) => Err(kratos_core_rs::error::Error::new(constant::ErrorCode::DataError as i32, &err.to_string())),
        }
    }
}
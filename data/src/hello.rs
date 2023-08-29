use super::Data;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct HelloRepo {
    pub data: Data,
}

pub fn new_hello_repo(data: Data) -> HelloRepo {
    return HelloRepo { data };
}

impl HelloRepo {
    #[allow(dead_code)]
    #[tracing::instrument]
    pub async fn create(&self, hello: Hello) -> Result<Hello, kratos_core_rs::error::Error> {
        let name = hello.name + " hello";

        Ok(Hello { name })
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("hello world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}

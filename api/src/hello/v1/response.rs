use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloResponse {
    pub name: String,
}


impl HelloResponse {
    pub fn from_biz(data: biz::hello::Hello) -> Self {
        HelloResponse { name: data.name }
    }
}

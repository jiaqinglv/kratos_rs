use super::Data;


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
    pub async fn create(&self, hello: Hello) -> Hello {
        let name = hello.name + " hello";

        Hello { name }
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

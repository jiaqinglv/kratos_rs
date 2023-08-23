use std::sync::Arc;

use service::WebServices;

use tonic::{Request, Response, Status};

use protos::hello_world_v1::{HelloRequest, HelloReply, self};
use protos::hello_world_v1::greeter_server::Greeter;

#[derive(Default)]
pub struct GreeterService {
    web_services: Arc<WebServices>,
}

impl GreeterService {
    #[allow(dead_code)]
    pub fn new(services: Arc<WebServices>) -> GreeterService {
        GreeterService { web_services:  services}
    }
}




#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("接收到请求");
        println!("Got a request from {:?}", request.remote_addr());

        let data = self.web_services.hello_service.create(request.into_inner().name).await;
        let reply = hello_world_v1::HelloReply {
            message: data.name.clone()
        };

        Ok(Response::new(reply))
    }
}


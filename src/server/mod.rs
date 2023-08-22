use crate::{
    kratos::server::Servers as KratosServers, 
    kratos::server::ServerAddress, 
    kratos_core_rs::core::server::{
        axum::AxumServer, 
        Server
    }, config::BootConfig
};
mod grpc;

// 服务实例
pub struct Servers {
    pub axum: Option<AxumServer>,
    pub grpc: Option<grpc::GrpcServer>
}

// 创建服务实例
pub async fn new_servers(conf: BootConfig)-> Result<Servers, Box<dyn std::error::Error>>  {
    Ok(
        Servers {
            axum: None,
            grpc: None,
        }.new(conf).await?
    )
}

// 需要实现框架的服务实例列表
impl KratosServers for Servers {
}

// 服务实例
impl Servers {
    pub async fn new(mut self, conf: BootConfig) -> Result<Self, Box<dyn std::error::Error>>{
        // 创建 axum 服务器
        self.new_grpc(conf).await?;

        Ok(self)
    }

    #[allow(dead_code)]
    pub async fn new_axum(&mut self, conf: BootConfig) -> Result<(), Box<dyn std::error::Error>>{
        // 创建http服务并进行监听
        let addr_str = conf.host.to_string() + ":" + &conf.port.to_string();
        let addr = (addr_str).parse()?;
        let axum_addr = ServerAddress::SocketAddr(addr);

        match axum_addr {
            // SocketAddr 类型
            ServerAddress::SocketAddr(addres) => {
                let server = AxumServer::bind(addres)?;
                self.axum = Some(server);
            }
            // 字符串类型
            ServerAddress::StringToSocketAddr(addres) => {
                let addr = addres.parse()?;
                let server = AxumServer::bind(addr)?;
                self.axum = Some(server);
            }
        };

        Ok(())
    }
    
    #[allow(dead_code)]
    pub async fn new_grpc(&mut self, conf:BootConfig) -> Result<(), Box<dyn std::error::Error>> {
        // 创建http服务并进行监听
        let addr_str = conf.host.to_string() + ":" + &conf.port.to_string();
        let addr = (addr_str).parse()?;
        let grpc_addr = ServerAddress::SocketAddr(addr);

        println!("{:#?}", conf);

        match grpc_addr {
            // SocketAddr 类型
            ServerAddress::SocketAddr(addres) => {
                let server = grpc::GrpcServer::bind(addres)?;
                self.grpc = Some(server);
            },
            // 字符串类型
            ServerAddress::StringToSocketAddr(addres) => {
                let addr = addres.parse()?;
                let server = grpc::GrpcServer::bind(addr)?;
                self.grpc = Some(server);
            }
        };

        Ok(())
    }

}

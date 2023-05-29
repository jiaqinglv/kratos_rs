use std::net::SocketAddr;

use axum::Router;

use super::Server;

pub struct AxumServer {
    // server: Option<Builder<hyper::AddrIncoming, Exec>>
    pub server: Option<hyper::server::Builder<hyper::server::conn::AddrIncoming>>,
}

impl Server for AxumServer {
    fn bind(addr: SocketAddr) -> Result<AxumServer, Box<dyn std::error::Error>> {
        let server = axum::Server::bind(&addr);

        Ok(AxumServer {
            server: Some(server),
        })
    }
}

impl AxumServer {
    #[allow(dead_code)]
    // 服务器监听运行
    pub async fn listen(self, router: Router) -> Result<(), Box<dyn std::error::Error>> {
        let server = match self.server {
            None => panic!("Axum Server is not listening"),
            Some(server) => server,
        };

        server.serve(router.into_make_service()).await?;

        Ok(())
    }
}

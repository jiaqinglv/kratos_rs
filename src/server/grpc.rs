use std::{
    pin::Pin,
    task::{Context, Poll},
    convert::Infallible,
};
use std::net::SocketAddr;
use futures::{future::{self, Either}, TryFutureExt};
use tonic::client::GrpcService;
use tower;
use http::version::Version;
use hyper::service::make_service_fn;


type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[allow(unused_imports, dead_code)]
enum EitherBody<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> http_body::Body for EitherBody<A, B>
where
    A: http_body::Body + Send + Unpin,
    B: http_body::Body<Data = A::Data> + Send + Unpin,
    A::Error: Into<Error>,
    B::Error: Into<Error>,
{
    type Data = A::Data;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn is_end_stream(&self) -> bool {
        match self {
            EitherBody::Left(b) => b.is_end_stream(),
            EitherBody::Right(b) => b.is_end_stream(),
        }
    }

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        match self.get_mut() {
            EitherBody::Left(b) => Pin::new(b).poll_data(cx).map(map_option_err),
            EitherBody::Right(b) => Pin::new(b).poll_data(cx).map(map_option_err),
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        match self.get_mut() {
            EitherBody::Left(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
            EitherBody::Right(b) => Pin::new(b).poll_trailers(cx).map_err(Into::into),
        }
    }
}

fn map_option_err<T, U: Into<Error>>(err: Option<Result<T, U>>) -> Option<Result<T, Error>> {
    err.map(|e| e.map_err(Into::into))
}

// GRPC 服务
#[derive(Debug)]
pub struct GrpcServer {
    pub grpc: Option<hyper::server::Builder<hyper::server::conn::AddrIncoming>>,
}

impl crate::server::Server for GrpcServer {
    // 绑定地址
    fn bind(addr: SocketAddr) -> Result<GrpcServer, Box<dyn std::error::Error>> {
        let grpc = hyper::Server::bind(&addr);
       
        Ok(GrpcServer {
            grpc: Some(grpc),
        })
    }
}

// GRPC Server 方法
impl GrpcServer {
    // 服务器监听运行
    #[allow(dead_code)]
    pub async fn listen<F>(self,mut new_router: F) -> Result<(), Box<dyn std::error::Error>> 
    where
        F: FnMut(&mut tonic::transport::Server) -> Result<(axum::Router,tonic::transport::server::Routes), Error>,
    {
        self.grpc.unwrap().serve(make_service_fn(move |_|{
            let mut grpc = tonic::transport::Server::builder();
            let (mut h,mut g) = new_router(&mut grpc).unwrap();

            future::ok::<_, Infallible>(tower::service_fn(
                move |req: hyper::Request<hyper::Body>| match req.version() {
                    Version::HTTP_11 | Version::HTTP_10 => {
                        println!("http 请求 {:#?}", req);

                        Either::Left(
                            // http 1.1、1.0协议走 axum 路由
                            h.call(req)
                            .map_ok(|res| res.map(EitherBody::Left))
                            .map_err(Error::from),
                        )
                    },
                    Version::HTTP_2 => {
                        println!("grpc 请求 {:#?}", req);
                        Either::Right(
                            // http2 协议走grpc
                            g.call(req)
                                .map_ok(|res| res.map(EitherBody::Right))
                                .map_err(Error::from),
                        )
                    },
                    _ => {
                        unimplemented!()
                    },
                },
            ))
        }))
        .await?;

        Ok(())
    }
}

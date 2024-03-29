#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
use std::{error::Error, sync::Arc};
use axum::Extension;
use tracing::{self, span,  info, instrument};
use opentelemetry::global;

use kratos_core_rs;
use configs as config;
use data;
use biz;
use service;
use protos as proto;
use api;


mod routers;
mod server;
mod wire;

use kratos_core_rs::core as kratos;
use kratos::logs::DefaultLogger;
use crate::{service::WebServices, wire::wire_app};


const VERSION: &'static str =  "0.0.1";
const APP_ID: &'static str = "app";
const APP_NAME: &'static str = "AppService";

#[tokio::main]
#[instrument(name = "main")]
async fn main() -> Result<(), Box<dyn Error>> {
    // 配置读取加载
    let conf = kratos::load_config::<config::BootConfig>(kratos::config::ConfigType::YAML(
        "./config/default.yaml".to_string(),
    )).await?;
    // 数据源加载
    let datas = data::new_data(&conf).await?;
    // 创建服务器实例
    let servers = server::new_servers(conf.clone()).await?;
    // 日志
    let logger = DefaultLogger::new_tracing_opentelemetry_jaeger(log::Level::Debug, APP_NAME.to_string()).expect("logger init faild!");
    // 创建应用
    let app = kratos::new_app(conf.clone(), datas, servers, logger, APP_ID, APP_NAME, VERSION);

    // web服务层
    let services: Arc<WebServices> = Arc::new(wire_app(&app.conf, app.data.unwrap()));
    // 路由
    let router = routers::new_http_router().layer(Extension(services.clone()));

    // 打印配置信息
    let conf = app.conf.clone();
    span!(tracing::Level::INFO, "config")
        .in_scope(|| {
            info!("server is running on http://{}:{}", conf.host, conf.port);
        });

    app
        .servers
        .expect("servers is None")
        .grpc
        .expect("grpc server is None")
        .listen(move |grpc:&mut tonic::transport::Server|  -> Result<
            (axum::Router, tonic::transport::server::Routes), 
            Box<dyn std::error::Error + Send + Sync + 'static>
        > {
            let gr = routers::new_grpc_router(grpc, services.clone());
            Ok((router.clone(), gr.into_service()))
        })
        .await?;


    global::shutdown_tracer_provider();
    Result::Ok(())
}

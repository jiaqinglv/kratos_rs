use tracing::Level;

use super::{
    config::ServerConfig,
    data::DataSource,
    log,
    server::Servers,
};

/// 服务器应用 C: 配置信息 S: 服务器
pub struct App<C, D, S>
where
    C: ServerConfig + Sized + Send + Sync,
    D: DataSource + Send + Sync,
    S: Servers + Send + Sync,
{
    // 应用ID
    pub id: String,
    // 名称
    pub name: String,
    // 版本
    pub version: String,
    // 自定义服务
    pub servers: Option<S>,
    // 配置文件
    pub conf: C,
    // 数据源
    pub data: Option<D>,
}

/// 应用信息
impl<C, D, S> App<C, D, S>
where
    C: ServerConfig + Sized + Sync + Send,
    D: DataSource + Send + Sync,
    S: Servers + Send + Sync,
{
    /// 日志初始化
    pub fn init_log(self, level: Level) -> Result<Self, Box<dyn std::error::Error>> {
        log::new_default_log(level).expect("init default log error");
        return Ok(self);
    }
}

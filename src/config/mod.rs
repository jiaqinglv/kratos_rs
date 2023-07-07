use serde::{Deserialize, Serialize};

use self::db::DBPoolConfig;
use crate::libs::core::config::ServerConfig;

pub mod db;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GRPCConfig {
    // 地址
    pub addr: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// 服务器配置文件
pub struct BootConfig {
    // 主机IP
    pub host: String,
    /// 端口
    pub port: i64,

    // pg 数据库配置
    pub pg: DBPoolConfig,
    // redia 数据库配置
    pub redis: DBPoolConfig,
}

impl ServerConfig for BootConfig {
    fn default() -> BootConfig {
        return BootConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
            pg: DBPoolConfig {
                max_size: 30,
                min_size: 10,
                idle_timeout: 100,
                connect_str: "postgresql://postgres:admin@localhost:5432/book".to_string(),
            },
            redis: DBPoolConfig {
                max_size: 30,
                min_size: 10,
                idle_timeout: 100,
                connect_str: "redis://default:admin@localhost:6379/db0".to_string(),
            },
        };
    }
}

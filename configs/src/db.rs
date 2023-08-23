use serde::{Deserialize, Serialize};

/// DataBase Pool Config
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DBPoolConfig {
    pub max_size: u32, // 池子最大大小
    pub min_size: u32, // 池子最小大小
    pub idle_timeout: u64,
    pub connect_str: String, // 连接字符串
}

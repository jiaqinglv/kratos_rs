use uuid::Uuid;

use snowflake::SnowflakeIdGenerator;

#[derive(Debug, Clone)]
pub struct IdGenerator {
    pub work_id: i32,
    pub node_id: i32,

    snowflake: SnowflakeIdGenerator,
}

impl IdGenerator {
    pub fn new(work_id: i32, node_id: i32) -> IdGenerator {
        let sg = SnowflakeIdGenerator::new(work_id, node_id);
        return IdGenerator{
            work_id,
            node_id,
            snowflake: sg
        };
    }

    /// 获取雪花数
    pub fn next_id(&mut self) -> i64 {
        self.snowflake.real_time_generate()
    }

    /// 新建uuid
    pub fn new_uuid(&self) -> uuid::Uuid {
        Uuid::new_v4()
    }

    /// 新建uuid String
    pub fn new_uuid_str(&self) -> String {
        Uuid::new_v4().to_string()
    }
}


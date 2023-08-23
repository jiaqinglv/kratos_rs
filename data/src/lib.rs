use  configs as config ;
use  kratos_core_rs::core::data::DataSource;

use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

pub mod hello;

#[derive(Debug, Clone)]
pub struct Data {
    pg_pool: Option<sqlx::Pool<sqlx::Postgres>>,
}

impl DataSource for Data {}

impl  Default for Data {
    fn default() -> Self {
        Self { pg_pool: None }
    }
}

#[derive(sqlx::FromRow, Debug)]
struct TimeNow {
    pub now: chrono::DateTime<chrono::Local>
}

pub async fn new_data(conf: &config::BootConfig) -> Result<Data, Box<dyn std::error::Error>> {
    println!("连接数据库中...");

    let mut d = Data {
        pg_pool: None,
    };

    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conf.pg.connect_str).await?;

    d.pg_pool = Some(pool);

    let time = sqlx::query_as::<_, TimeNow>(
            "SELECT now()"
        ).fetch_one(&d.pg_pool.clone().unwrap()).await?;
        

    println!("{} INIT 连接数据库连接成功[OK]  ", time.now);

    return Ok(d);
}

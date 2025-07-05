use diesel::{r2d2::ConnectionManager, MysqlConnection};
use env_logger::Env;
use log::debug;
use diesel_migrations::MigrationHarness;
use crate::db::migrations::MIGRATIONS;

use crate::{db::model_dao::Dao, zig_config::ZigConfig, zig_error::ZigAnyResult};
use std::sync::Arc;
mod db;

mod zig_config;
mod zig_error;
mod http_server;

pub struct ZigService {
    pub dao: Arc<Dao>,
    pub config: Arc<ZigConfig>,
    pub http_client: Arc<reqwest::Client>,
}

#[actix_web::main]
async fn main() -> ZigAnyResult<()>{

    ZigConfig::init();

    let service_config = ZigConfig::new()?;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let manager = ConnectionManager::<MysqlConnection>::new(service_config.db_url.clone());
    let pool = diesel::r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to build MySQL connection pool");
    debug!("MySQL db pool setup");

    let dao = Dao::new(&pool);

    let zig_service = Arc::new(ZigService {
        dao: Arc::new(dao),
        config: Arc::new(service_config),
        http_client: Arc::new(reqwest::Client::new()),
    });
    
    {
        let mut conn = zig_service.dao.pool.get().expect("Failed to get connection for database migrations");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run database migrations");
    }
    debug!("successfully ran database migrations");

    debug!("Starting http server...");
    http_server::start_http_server(zig_service).await?;
    Ok(())
}

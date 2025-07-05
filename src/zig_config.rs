use std::env;
use dotenv::dotenv;
use crate::zig_error::{ZigAnyResult};

pub struct ZigConfig {
    pub db_url: String,
}

impl ZigConfig {

    pub fn init() {
        dotenv().ok();
    }

    pub fn new() -> ZigAnyResult<Self> {
        Ok(ZigConfig {
            db_url: env::var("DATABASE_URL")
                .expect("env DATABASE_URL is required"),
        })
    }
}
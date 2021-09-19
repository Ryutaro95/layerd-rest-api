use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct DBContext {
    pub pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl DBContext {
    pub fn new() -> DBContext {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB connection pool.");
        DBContext { pool }
    }
}
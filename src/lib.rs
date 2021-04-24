extern crate dotenv;
use sqlx::{MySqlPool, Result};

use dotenv::dotenv;
use std::env;

pub mod models;

pub async fn establish_connection() -> Result<MySqlPool> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&database_url).await
}

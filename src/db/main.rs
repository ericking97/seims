use core::panic;
use std::env;

use sqlx::postgres::PgPoolOptions;

const DATABASE_ENV: &str = "INVENTORY_MANAGEMENT_DB";

pub async fn create_connection() -> sqlx::Pool<sqlx::Postgres> {
    let database_string = match env::var(DATABASE_ENV) {
        Ok(v) => v,
        Err(err) => panic!("${} is not set ({})", DATABASE_ENV, err),
    };

    let result = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_string)
        .await;

    let pool = match result {
        Ok(p) => p,
        Err(err) => panic!("Cannot connect with the database {:?}", err),
    };

    pool
}

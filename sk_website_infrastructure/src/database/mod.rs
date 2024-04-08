use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub mod sqlx_user_repository;

pub async fn get_pool() -> Pool<Postgres> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await;

    pool.unwrap()
}

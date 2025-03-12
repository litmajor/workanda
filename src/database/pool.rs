use sqlx::PgPool;
use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;

pub type DbPool = Arc<PgPool>;
pub type Pool = sqlx::PgPool;


pub type Pool = sqlx::PgPool;

pub async fn establish_connection_pool() -> Result<Pool, sqlx::Error> {

dotenv().ok();

let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

PgPoolOptions::new()

.max_connections(5)

.connect(&database_url)

.await

}
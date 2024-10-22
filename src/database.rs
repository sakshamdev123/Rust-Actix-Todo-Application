use sqlx::MySqlPool;
use std::env;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(&env::var("MYSQL_URI").unwrap()).await
}

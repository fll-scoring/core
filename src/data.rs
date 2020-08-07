use super::config::get_global_value;
use sqlx::postgres::PgPool;
use super::errors::ServiceError;

pub async fn get_db_pool() -> Result<PgPool, super::errors::ServiceError> {
  let db_url = get_global_value("postgres-url", true)?;
  let pool = PgPool::builder()
    .max_size(5u32)
    .build(db_url.as_str()).await;

  match pool {
    Ok(pool) => Ok(pool),
    Err(_) => Err(ServiceError::InternalServerError("Unable to connect to the Postgres database".to_string()))
  }
}



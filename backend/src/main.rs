mod models;

use std::env;

type AllResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> AllResult<()> {
    dotenvy::dotenv()?;
    let url = env::var("DATABASE_URL")?;
    let connection = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!().run(&connection).await?;

    Ok(())
}

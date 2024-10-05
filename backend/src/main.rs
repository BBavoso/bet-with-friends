mod models;
mod repositories;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let url = env::var("DATABASE_URL")?;
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!().run(&pool).await?;

    let users = repositories::users::read(&pool).await?;

    println!("{:?}", users);

    Ok(())
}

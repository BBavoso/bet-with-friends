mod models;
mod repositories;

use std::env;

type AllResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> AllResult<()> {
    dotenvy::dotenv()?;
    let url = env::var("DATABASE_URL")?;
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!().run(&pool).await?;

    let user = repositories::users::get_user_with_id(&pool, 1).await?;

    println!("{:?}", user);

    Ok(())
}

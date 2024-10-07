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

    let user = repositories::users::create_user(
        &pool,
        "carlos".into(),
        "carlos@mail.com".into(),
        "pass4321".into(),
    )
    .await?;

    println!("{:?}", user);

    Ok(())
}

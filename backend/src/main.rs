mod models;
mod router;

use std::env;

type AllResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> AllResult<()> {
    dotenvy::dotenv()?;
    let url = env::var("DATABASE_URL")?;
    let connection = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!().run(&connection).await?;

    let app: axum::Router = router::create_router(connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

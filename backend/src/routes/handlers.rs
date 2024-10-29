use crate::models::User;
use axum::{extract::State, Json};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct UserCreator {
    username: String,
    email: String,
    password_hash: String,
}

pub async fn create_user(
    pool: State<PgPool>,
    body: Json<UserCreator>,
) -> Result<Json<User>, Box<str>> {
    let UserCreator {
        username,
        email,
        password_hash,
    } = body.0;

    User::new(&pool, username, email, password_hash)
        .await
        .map(|user| Json(user))
        .map_err(|_| Box::from("Database Error"))
}

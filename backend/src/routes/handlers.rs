use crate::models::User;
use axum::{extract::State, Json};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password_hash: String,
}

pub async fn create_user(
    pool: State<PgPool>,
    body: Json<CreateUser>,
) -> Result<Json<User>, Box<str>> {
    let CreateUser {
        username,
        email,
        password_hash,
    } = body.0;

    User::new(&pool, username, email, password_hash)
        .await
        .map(|user| Json(user))
        .map_err(|_| Box::from("Unable to create user"))
}

#[derive(Deserialize)]
pub struct GetUser {
    username: String,
}

pub async fn get_user(pool: State<PgPool>, body: Json<GetUser>) -> Result<Json<User>, Box<str>> {
    let GetUser { username } = body.0;

    User::read_from_name(&pool, &username)
        .await
        .map(|user| Json(user))
        .map_err(|_| Box::from("Unable to get user"))
}

use crate::models::{Score, User};
use axum::{extract::State, Json};
use serde::Deserialize;
use sqlx::PgPool;

type APIResult<T> = Result<Json<T>, &'static str>;

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,
}

pub async fn create_user(pool: State<PgPool>, body: Json<CreateUser>) -> APIResult<User> {
    let CreateUser {
        username,
        email,
        password,
    } = body.0;

    User::new(&pool, username, email, password)
        .await
        .map(|user| Json(user))
        .map_err(|_| "Unable to create user")
}

#[derive(Deserialize)]
pub struct Username {
    username: String,
}

pub async fn get_user(pool: State<PgPool>, body: Json<Username>) -> APIResult<User> {
    let Username { username } = body.0;
    User::read_from_name(&pool, &username)
        .await
        .map(|user| Json(user))
        .map_err(|_| "Unable to get user")
}

pub async fn get_score(
    State(pool): State<PgPool>,
    Json(Username { username }): Json<Username>,
) -> APIResult<Score> {
    Score::from_username(&pool, &username)
        .await
        .map(|user| Json(user))
        .map_err(|_| "Unable to get score")
}

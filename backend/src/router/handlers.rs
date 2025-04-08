use crate::models::{Bet, Score, User};
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

pub async fn create_user(
    pool: State<PgPool>,
    Json(CreateUser {
        username,
        email,
        password,
    }): Json<CreateUser>,
) -> APIResult<User> {
    User::new(&pool, username, email, password)
        .await
        .map(|user| Json(user))
        .map_err(|_| "Unable to create user")
}

#[derive(Deserialize)]
pub struct Username {
    username: String,
}

pub async fn get_user(
    pool: State<PgPool>,
    Json(Username { username }): Json<Username>,
) -> APIResult<User> {
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

#[derive(Deserialize)]
pub struct CreateBet {
    username: String,
    description: String,
    stop_bets_at: Option<chrono::NaiveDateTime>,
}

pub async fn create_bet(
    State(pool): State<PgPool>,
    Json(CreateBet {
        username,
        description,
        stop_bets_at,
    }): Json<CreateBet>,
) -> APIResult<Bet> {
    let user = User::read_from_name(&pool, &username)
        .await
        .map_err(|_| "Unable to get user")?;
    let bet = match stop_bets_at {
        Some(time) => user.create_timed_bet(&pool, description, time).await,
        None => user.create_timeless_bet(&pool, description).await,
    };
    bet.map(|bet| Json(bet)).map_err(|_| "Unable to create bet")
}

pub async fn get_bets(
    State(pool): State<PgPool>,
    Json(Username { username }): Json<Username>,
) -> APIResult<Vec<Bet>> {
    let user = User::read_from_name(&pool, &username)
        .await
        .map_err(|_| "Unable to get user")?;
    user.bets_created(&pool)
        .await
        .map(|bet| Json(bet))
        .map_err(|_| "Unable to get bets")
}

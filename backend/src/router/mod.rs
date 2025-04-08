mod handlers;

use axum::routing::{get, post};
use handlers::{create_bet, create_user, get_bets, get_score, get_user};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> axum::Router {
    axum::Router::new()
        .route("/user", post(create_user))
        .route("/user", get(get_user))
        .route("/user/score", get(get_score))
        .route("/user/bets", get(get_bets))
        .route("/bet", post(create_bet))
        .with_state(pool)
}

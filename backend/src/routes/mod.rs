mod handlers;

use axum::routing::post;
use handlers::create_user;
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> axum::Router {
    axum::Router::new().route("/user", post(create_user).with_state(pool))
}

mod handlers;

use axum::routing::{get, post};
use handlers::{create_user, get_user};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> axum::Router {
    axum::Router::new()
        .route("/user", post(create_user))
        .route("/user", get(get_user))
        .with_state(pool)
}

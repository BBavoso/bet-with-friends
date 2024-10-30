use serde::Serialize;
use sqlx::PgPool;

use crate::AllResult;

use super::repositories::scores;

#[derive(Debug, PartialEq, Serialize)]
pub struct Score {
    pub user_id: i32,
    pub total_wins: i32,
    pub total_losses: i32,
    pub points_earned: i32,
}

impl Score {
    pub async fn from_username(connection: &PgPool, username: &str) -> AllResult<Score> {
        scores::read_score_by_username(connection, username).await
    }
}

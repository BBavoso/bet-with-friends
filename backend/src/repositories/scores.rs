#![allow(dead_code)]

use sqlx::PgPool;

use crate::{
    models::{Score, User},
    AllResult,
};

pub async fn create_default_score(connection: &PgPool, user: &User) -> AllResult<Score> {
    let score = sqlx::query_as!(
        Score,
        r#"
        INSERT INTO scores (user_id, total_wins, total_losses, points_earned)
        VALUES ($1, 0, 0, 0)
        RETURNING user_id, total_wins, total_losses, points_earned
        "#,
        user.id,
    )
    .fetch_one(connection)
    .await?;
    Ok(score)
}

pub async fn read_user_score(connection: &PgPool, user: &User) -> AllResult<Score> {
    let score = sqlx::query_as!(
        Score,
        r#"
        SELECT user_id, total_wins, total_losses, points_earned
        FROM scores WHERE user_id = $1
        "#,
        user.id,
    )
    .fetch_one(connection)
    .await?;
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    use crate::repositories::users::create_users;

    #[sqlx::test]
    async fn read_default_score(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob"]).await?;
        let bob = users.pop().unwrap();

        let created_score = create_default_score(&pool, &bob).await?;
        let read_score = read_user_score(&pool, &bob).await?;

        assert_eq!(created_score, read_score);

        Ok(())
    }
}

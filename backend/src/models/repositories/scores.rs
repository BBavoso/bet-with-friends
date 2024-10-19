#![allow(dead_code)]

use sqlx::PgPool;

use crate::{
    models::{BetParticipant, Score, User},
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

pub async fn update_score_winning_bet(
    connection: &PgPool,
    participant: &BetParticipant,
) -> AllResult<Score> {
    let score = sqlx::query_as!(
        Score,
        r#"
        UPDATE scores
        SET total_wins = total_wins + 1, points_earned = $1
        WHERE user_id = $2
        RETURNING *
        "#,
        participant.bet_amount,
        participant.user_id
    )
    .fetch_one(connection)
    .await?;
    Ok(score)
}

pub async fn update_score_losing_bet(
    connection: &PgPool,
    participant: &BetParticipant,
) -> AllResult<Score> {
    let score = sqlx::query_as!(
        Score,
        r#"
        UPDATE scores
        SET total_losses = total_losses + 1
        WHERE user_id = $1
        RETURNING *
        "#,
        participant.user_id
    )
    .fetch_one(connection)
    .await?;
    Ok(score)
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use sqlx::PgPool;

    use super::super::{
        bet_participants::create_bet_participant, bets::create_timeless_bet, users::create_users,
    };

    #[sqlx::test]
    async fn read_default_score(pool: PgPool) -> AllResult<()> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING *;
            "#,
            "bob",
            "bob@mail.com",
            "bobpass"
        )
        .fetch_one(&pool)
        .await?;

        let created_score = create_default_score(&pool, &user).await?;
        let read_score = read_user_score(&pool, &user).await?;

        assert_eq!(created_score, read_score);
        assert_eq!(created_score.points_earned, 0);
        assert_eq!(created_score.total_wins, 0);
        assert_eq!(created_score.total_losses, 0);
        assert_eq!(created_score.user_id, user.id);

        Ok(())
    }

    #[sqlx::test]
    async fn bet_win(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob"]).await?;
        let bob = users.pop().unwrap();

        let score = read_user_score(&pool, &bob).await?;

        assert_eq!(score.points_earned, 0);
        assert_eq!(score.total_wins, 0);
        assert_eq!(score.total_losses, 0);

        let bet = create_timeless_bet(&pool, &bob, "".into()).await?;
        let bet_participant = create_bet_participant(&pool, &bob, &bet, 100, true).await?;

        let score = update_score_winning_bet(&pool, &bet_participant).await?;

        assert_eq!(score.points_earned, 100);
        assert_eq!(score.total_wins, 1);
        assert_eq!(score.total_losses, 0);

        Ok(())
    }

    #[sqlx::test]
    async fn bet_loss(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob"]).await?;
        let bob = users.pop().unwrap();

        let score = read_user_score(&pool, &bob).await?;

        assert_eq!(score.points_earned, 0);
        assert_eq!(score.total_wins, 0);
        assert_eq!(score.total_losses, 0);

        let bet = create_timeless_bet(&pool, &bob, "".into()).await?;
        let bet_participant = create_bet_participant(&pool, &bob, &bet, 100, true).await?;

        let score = update_score_losing_bet(&pool, &bet_participant).await?;

        assert_eq!(score.points_earned, 0);
        assert_eq!(score.total_wins, 0);
        assert_eq!(score.total_losses, 1);

        Ok(())
    }
}

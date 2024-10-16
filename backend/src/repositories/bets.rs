#![allow(dead_code)]

use sqlx::types::chrono::NaiveDateTime;

use crate::models::{Bet, BetStatus, User};
use crate::AllResult;

pub async fn get_bet_by_id(connection: &sqlx::PgPool, id: i32) -> AllResult<Bet> {
    let bet = sqlx::query_as!(
        Bet,
        r#"
        SELECT id, creator_id, description, status AS "status: BetStatus",
        stop_bets_at, created_at, updated_at, paid_out, paid_out_at
        FROM bets WHERE id = $1
        "#,
        id,
    )
    .fetch_one(connection)
    .await?;
    Ok(bet)
}

pub async fn get_bets_by_status(connection: &sqlx::PgPool, status: &BetStatus) -> AllResult<Bet> {
    let bet = sqlx::query_as!(
        Bet,
        r#"
        SELECT id, creator_id, description, status AS "status: BetStatus",
        stop_bets_at, created_at, updated_at, paid_out, paid_out_at
        FROM bets WHERE status = $1
        "#,
        status as _,
    )
    .fetch_one(connection)
    .await?;
    Ok(bet)
}

pub async fn create_timeless_bet(
    connection: &sqlx::PgPool,
    user: &User,
    description: String,
) -> AllResult<Bet> {
    let bet = sqlx::query_as!(
        Bet,
        r#"
        INSERT INTO bets (creator_id, description, status, paid_out)
        VALUES ($1, $2, $3, FALSE)
        RETURNING id, creator_id, description, status AS "status: BetStatus",
        stop_bets_at, created_at, updated_at, paid_out, paid_out_at
        "#,
        user.id,
        description,
        BetStatus::Active as _
    )
    .fetch_one(connection)
    .await?;
    Ok(bet)
}

pub async fn create_timed_bet(
    connection: &sqlx::PgPool,
    user: &User,
    description: String,
    stop_bets_at: NaiveDateTime,
) -> AllResult<Bet> {
    let bet = sqlx::query_as!(
        Bet,
        r#"
        INSERT INTO bets (creator_id, description, status, paid_out, stop_bets_at)
        VALUES ($1, $2, $3, FALSE, $4)
        RETURNING id, creator_id, description, status AS "status: BetStatus",
        stop_bets_at, created_at, updated_at, paid_out, paid_out_at
        "#,
        user.id,
        description,
        BetStatus::Active as _,
        stop_bets_at
    )
    .fetch_one(connection)
    .await?;
    Ok(bet)
}

pub async fn close_bet(connection: &sqlx::PgPool, bet: Bet) -> AllResult<Bet> {
    assert_eq!(bet.status, BetStatus::Active);
    assert_eq!(bet.stop_bets_at, None);
    let bet = sqlx::query_as!(
        Bet,
        r#"
        UPDATE bets
        SET status = $1
        WHERE id = $2
        RETURNING id, creator_id, description, status AS "status: BetStatus",
        stop_bets_at, created_at, updated_at, paid_out, paid_out_at
        "#,
        BetStatus::Finished as _,
        bet.id
    )
    .fetch_one(connection)
    .await?;
    Ok(bet)
}

pub async fn payout_bet(connection: &sqlx::PgPool, bet: Bet) -> AllResult<Bet> {
    assert_eq!(bet.status, BetStatus::Finished);
    todo!("Need to payout each bet participant");
    let bet = sqlx::query_as!(
        Bet,
        r#"
        UPDATE bets
        SET status = $1
        WHERE id = $2
        RETURNING id, creator_id, description, status AS "status: BetStatus",
        stop_bets_at, created_at, updated_at, paid_out, paid_out_at
        "#,
        BetStatus::PayedOut as _,
        bet.id
    )
    .fetch_one(connection)
    .await?;
    Ok(bet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::users::create_users;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn create_and_read_bet(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob"]).await?;
        let bob = users.pop().unwrap();

        let created_bet =
            create_timeless_bet(&pool, &bob, String::from("test_description")).await?;
        assert_eq!(created_bet.creator_id, bob.id);
        assert_eq!(created_bet.description, String::from("test_description"));
        assert_eq!(created_bet.paid_out, false);
        assert_eq!(created_bet.status, BetStatus::Active);

        let read_bet = get_bet_by_id(&pool, bob.id).await?;
        assert_eq!(created_bet, read_bet);

        Ok(())
    }

    #[sqlx::test]
    async fn run_bet_no_participants(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob"]).await?;
        let bob = users.pop().unwrap();

        let bet = create_timeless_bet(&pool, &bob, String::from("test_description")).await?;

        assert_eq!(bet.status, BetStatus::Active);

        let bet_copy = bet.clone();
        let bet = close_bet(&pool, bet).await?;

        assert_eq!(bet_copy.id, bet.id);
        assert_eq!(bet_copy.creator_id, bet.creator_id);
        assert_ne!(bet_copy.updated_at, bet.updated_at);
        assert_eq!(bet.status, BetStatus::Finished);

        let bet_copy = bet.clone();
        let bet = payout_bet(&pool, bet).await?;

        assert_eq!(bet_copy.id, bet.id);
        assert_eq!(bet_copy.creator_id, bet.creator_id);
        assert_ne!(bet_copy.updated_at, bet.updated_at);
        assert_eq!(bet.status, BetStatus::PayedOut);

        Ok(())
    }

    #[sqlx::test]
    async fn run_bet_with_participants(pool: PgPool) -> AllResult<()> {
        todo!()
    }
}

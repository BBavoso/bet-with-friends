#![allow(dead_code, unreachable_code, unused_variables)]

use sqlx::types::chrono::NaiveDateTime;

use crate::models::{Bet, BetParticipant, BetStatus, User};
use crate::repositories::bet_participants::{get_bet_participants, payout_participant};
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

pub async fn get_bets_by_status(
    connection: &sqlx::PgPool,
    status: &BetStatus,
) -> AllResult<Vec<Bet>> {
    let bet = sqlx::query_as!(
        Bet,
        r#"
        SELECT id, creator_id, description, status AS "status: BetStatus",
        stop_bets_at, created_at, updated_at, paid_out, paid_out_at
        FROM bets WHERE status = $1
        "#,
        status as _,
    )
    .fetch_all(connection)
    .await?;
    Ok(bet)
}

pub async fn get_bets_by_user(connection: &sqlx::PgPool, user: &User) -> AllResult<Vec<Bet>> {
    let bet = sqlx::query_as!(
        Bet,
        r#"
        SELECT id, creator_id, description, status AS "status: BetStatus",
        stop_bets_at, created_at, updated_at, paid_out, paid_out_at
        FROM bets WHERE creator_id = $1
        "#,
        user.id,
    )
    .fetch_all(connection)
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

pub async fn payout_bet(connection: &sqlx::PgPool, bet: Bet, for_bet: bool) -> AllResult<Bet> {
    assert_eq!(bet.status, BetStatus::Finished);
    let participants_to_payout = get_bet_participants(connection, &bet).await?;
    for participant in participants_to_payout {
        payout_participant(connection, participant, for_bet).await?;
    }

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

pub async fn get_bets_with_user(
    connection: &sqlx::PgPool,
    user: &User,
) -> AllResult<Vec<(Bet, BetParticipant)>> {
    let result = sqlx::query!(
        r#"
        SELECT
            bet_id, user_id, for_bet, bet_amount, participants.paid_out AS participant_paid,
            id, creator_id, description, status AS "status: BetStatus", stop_bets_at, created_at, updated_at, bets.paid_out, paid_out_at
        FROM bet_participants AS participants JOIN bets ON bet_id = id WHERE user_id = $1;
        "#,
        user.id
    ).map(|row| (
        Bet {
            id: row.id,
            creator_id: row.creator_id,
            description: row.description,
            status: row.status,
            stop_bets_at: row.stop_bets_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
            paid_out: row.paid_out,
            paid_out_at: row.paid_out_at,
        },
        BetParticipant {
            bet_id: row.bet_id,
            user_id: row.user_id,
            for_bet: row.for_bet,
            bet_amount: row.bet_amount,
            paid_out: row.participant_paid,
        },
    ))
        .fetch_all(connection)
        .await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{bet_participants, users::create_users};
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
        let bet = payout_bet(&pool, bet, true).await?;

        assert_eq!(bet_copy.id, bet.id);
        assert_eq!(bet_copy.creator_id, bet.creator_id);
        assert_ne!(bet_copy.updated_at, bet.updated_at);
        assert_eq!(bet.status, BetStatus::PayedOut);

        Ok(())
    }

    #[sqlx::test]
    async fn run_bet_with_participants(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob", "John"]).await?;
        let bob = users.pop().unwrap();
        let john = users.pop().unwrap();

        let timeless_bet = create_timeless_bet(&pool, &bob, String::from("description")).await?;
        create_timeless_bet(&pool, &bob, String::from("description")).await?;
        create_timeless_bet(&pool, &bob, String::from("description")).await?;
        create_timeless_bet(&pool, &john, String::from("description")).await?;

        let now = sqlx::types::chrono::Local::now().naive_local();
        let tommorow = now + chrono::TimeDelta::days(1);

        let timed_bet =
            create_timed_bet(&pool, &bob, String::from("description"), tommorow).await?;
        create_timed_bet(&pool, &bob, String::from("description"), tommorow).await?;
        create_timed_bet(&pool, &bob, String::from("description"), tommorow).await?;
        create_timed_bet(&pool, &john, String::from("description"), tommorow).await?;

        let bets = get_bets_by_user(&pool, &bob).await?;
        assert_eq!(bets.len(), 6);

        let bob_timeless_bet =
            bet_participants::create_bet_participant(&pool, &bob, &timeless_bet, 10, true).await?;
        let john_timeless_bet =
            bet_participants::create_bet_participant(&pool, &john, &timeless_bet, 10, true).await?;
        let bob_timed_bet =
            bet_participants::create_bet_participant(&pool, &bob, &timed_bet, 10, true).await?;
        let john_timed_bet =
            bet_participants::create_bet_participant(&pool, &john, &timed_bet, 10, true).await?;

        let user_bets = get_bets_with_user(&pool, &bob).await?;

        assert_eq!(user_bets.len(), 2);
        assert!(
            user_bets
                .iter()
                .any(|(bet, bet_participant)| bet == &timeless_bet
                    && bet_participant.bet_id == bet.id)
        );
        assert_eq!(user_bets.len(), 2);
        assert!(user_bets
            .iter()
            .any(|(bet, bet_participant)| bet == &timed_bet && bet_participant.bet_id == bet.id));

        let timed_bet = close_bet(&pool, timed_bet).await?;
        let timeless_bet = close_bet(&pool, timeless_bet).await?;

        let payed_out_timeless = payout_bet(&pool, timeless_bet.clone(), true).await?;
        assert_eq!(payed_out_timeless.status, BetStatus::PayedOut);
        assert_eq!(payed_out_timeless.id, timeless_bet.id);

        let payed_out_timed = payout_bet(&pool, timed_bet.clone(), true).await?;
        assert_eq!(payed_out_timed.status, BetStatus::PayedOut);
        assert_eq!(payed_out_timed.id, timed_bet.id);

        Ok(())
    }
}

#![allow(dead_code, unreachable_code, unused_variables)]

use crate::models::{Bet, BetParticipant, User};
use crate::AllResult;

use super::scores;

pub async fn get_bet_participant_by_bet_id(
    connection: &sqlx::PgPool,
    bet_id: i32,
) -> AllResult<BetParticipant> {
    let bet_participant = sqlx::query_as!(
        BetParticipant,
        r#"
        SELECT bet_id, user_id, for_bet, bet_amount, paid_out
        FROM bet_participants WHERE bet_id = $1
        "#,
        bet_id,
    )
    .fetch_one(connection)
    .await?;
    Ok(bet_participant)
}

pub async fn create_bet_participant(
    connection: &sqlx::PgPool,
    user: &User,
    bet: &Bet,
    amount: i32,
    for_bet: bool,
) -> AllResult<BetParticipant> {
    let bet_participant = sqlx::query_as!(
        BetParticipant,
        r#"
        INSERT INTO bet_participants (bet_id, user_id, for_bet, bet_amount, paid_out) VALUES ($1, $2, $3, $4, FALSE) RETURNING *;
        "#,
        bet.id,
        user.id,
        for_bet,
        amount
    ).fetch_one(connection).await?;
    Ok(bet_participant)
}

pub async fn get_bet_participants(
    connection: &sqlx::PgPool,
    bet: &Bet,
) -> AllResult<Vec<BetParticipant>> {
    let bet_participants = sqlx::query_as!(
        BetParticipant,
        r#"
        SELECT * FROM bet_participants WHERE bet_id = $1
        "#,
        bet.id
    )
    .fetch_all(connection)
    .await?;
    Ok(bet_participants)
}

pub async fn payout_participant(
    connection: &sqlx::PgPool,
    participant: BetParticipant,
    for_bet: bool,
) -> AllResult<BetParticipant> {
    let participant = sqlx::query_as!(
        BetParticipant,
        r#"
        UPDATE bet_participants
        SET paid_out = TRUE
        WHERE bet_id = $1 AND user_id = $2
        RETURNING *
        "#,
        participant.bet_id,
        participant.user_id
    )
    .fetch_one(connection)
    .await?;
    println!("adding to score {:?}", participant);
    scores::add_to_score(connection, &participant).await?;
    println!("added score {:?}", participant);
    return Ok(participant);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{
        bet_participants,
        bets::{create_timed_bet, create_timeless_bet},
        users::create_users,
    };
    use sqlx::PgPool;

    #[sqlx::test]
    async fn get_participants_of_bet(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["bob"]).await?;
        let bob = users.pop().unwrap();

        let mut users = create_users(&pool, vec!["Bob", "John"]).await?;
        let bob = users.pop().unwrap();
        let john = users.pop().unwrap();

        let timeless_bet = create_timeless_bet(&pool, &bob, String::from("description")).await?;

        let now = sqlx::types::chrono::Local::now().naive_local();
        let tommorow = now + chrono::TimeDelta::days(1);

        let timed_bet =
            create_timed_bet(&pool, &bob, String::from("description"), tommorow).await?;

        let bob_timeless_bet =
            bet_participants::create_bet_participant(&pool, &bob, &timeless_bet, 10, true).await?;
        let john_timeless_bet =
            bet_participants::create_bet_participant(&pool, &john, &timeless_bet, 10, true).await?;
        let bob_timed_bet =
            bet_participants::create_bet_participant(&pool, &bob, &timed_bet, 10, true).await?;
        let john_timed_bet =
            bet_participants::create_bet_participant(&pool, &john, &timed_bet, 10, true).await?;

        assert_eq!(get_bet_participants(&pool, &timeless_bet).await?.len(), 2);
        assert_eq!(get_bet_participants(&pool, &timed_bet).await?.len(), 2);

        Ok(())
    }
}

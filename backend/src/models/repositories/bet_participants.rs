use sqlx::PgPool;

use crate::models::{Bet, BetParticipant, Score, User};
use crate::AllResult;

use super::scores;

pub async fn get_bet_participant_by_bet_id(
    connection: &PgPool,
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

pub async fn get_bet_participants_by_bet_user(
    connection: &PgPool,
    user: &User,
) -> AllResult<Vec<BetParticipant>> {
    let bet_participant = sqlx::query_as!(
        BetParticipant,
        r#"
        SELECT bet_id, user_id, for_bet, bet_amount, paid_out
        FROM bet_participants WHERE user_id = $1
        "#,
        user.id,
    )
    .fetch_all(connection)
    .await?;
    Ok(bet_participant)
}

pub async fn create_bet_participant(
    connection: &PgPool,
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
    connection: &PgPool,
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

pub(crate) async fn payout_participant(
    connection: &PgPool,
    participant: BetParticipant,
    bet_outcome: bool,
) -> AllResult<(BetParticipant, Score)> {
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
    let win = bet_outcome == participant.for_bet;
    let score = match win {
        true => scores::update_score_winning_bet(connection, &participant).await?,
        false => scores::update_score_losing_bet(connection, &participant).await?,
    };
    Ok((participant, score))
}

#[cfg(test)]
mod tests {
    use super::super::{
        bet_participants,
        bets::{create_timed_bet, create_timeless_bet},
        users::create_users,
    };
    use super::*;
    use bet_participants::create_bet_participant;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn get_participants_of_bet(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob", "John"]).await?;
        let bob = users.pop().unwrap();
        let john = users.pop().unwrap();

        let timeless_bet = create_timeless_bet(&pool, &bob, String::from("description")).await?;

        let now = sqlx::types::chrono::Local::now().naive_local();
        let tommorow = now + chrono::TimeDelta::days(1);

        let timed_bet =
            create_timed_bet(&pool, &bob, String::from("description"), tommorow).await?;

        let bob_timeless_bet = create_bet_participant(&pool, &bob, &timeless_bet, 10, true).await?;
        let john_timeless_bet =
            create_bet_participant(&pool, &john, &timeless_bet, 10, true).await?;
        let bob_timed_bet = create_bet_participant(&pool, &bob, &timed_bet, 10, true).await?;
        let john_timed_bet = create_bet_participant(&pool, &john, &timed_bet, 10, true).await?;

        assert_eq!(bob_timeless_bet.bet_id, timeless_bet.id);
        assert_eq!(john_timeless_bet.bet_id, timeless_bet.id);
        assert_eq!(bob_timed_bet.bet_id, timed_bet.id);
        assert_eq!(john_timed_bet.bet_id, timed_bet.id);

        assert_eq!(get_bet_participants(&pool, &timeless_bet).await?.len(), 2);
        assert_eq!(get_bet_participants(&pool, &timed_bet).await?.len(), 2);

        Ok(())
    }

    #[sqlx::test]
    async fn payout_participant_of_bet(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["bob", "john"]).await?;
        let bob = users.pop().unwrap();
        let john = users.pop().unwrap();

        let bet = create_timeless_bet(&pool, &bob, String::from("description")).await?;

        let bob_bet = create_bet_participant(&pool, &bob, &bet, 10, true).await?;
        let john_bet = create_bet_participant(&pool, &john, &bet, 25, false).await?;

        let (bob_bet, bob_score) = payout_participant(&pool, bob_bet, true).await?;
        assert_eq!(bob_bet.paid_out, true);
        assert_eq!(bob_score.points_earned, 10);
        assert_eq!(bob_score.total_wins, 1);
        assert_eq!(bob_score.total_losses, 0);

        let (john_bet, john_score) = payout_participant(&pool, john_bet, true).await?;
        assert_eq!(john_bet.paid_out, true);
        assert_eq!(john_score.points_earned, 0);
        assert_eq!(john_score.total_wins, 0);
        assert_eq!(john_score.total_losses, 1);

        Ok(())
    }

    #[sqlx::test]
    async fn particpate_in_bets(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob", "John"]).await?;
        let bob = users.pop().unwrap();
        let john = users.pop().unwrap();

        let bet1 = create_timeless_bet(&pool, &bob, String::from("description")).await?;
        let bet2 = create_timeless_bet(&pool, &bob, String::from("description")).await?;
        let bet3 = create_timeless_bet(&pool, &bob, String::from("description")).await?;

        create_bet_participant(&pool, &bob, &bet1, 10, true).await?;
        create_bet_participant(&pool, &bob, &bet2, 10, true).await?;
        create_bet_participant(&pool, &bob, &bet3, 10, true).await?;
        create_bet_participant(&pool, &john, &bet1, 10, true).await?;
        create_bet_participant(&pool, &john, &bet2, 10, true).await?;

        assert_eq!(
            get_bet_participants_by_bet_user(&pool, &bob).await?.len(),
            3
        );

        assert_eq!(
            get_bet_participants_by_bet_user(&pool, &john).await?.len(),
            2
        );

        Ok(())
    }
}

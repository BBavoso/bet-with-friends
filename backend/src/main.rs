mod models;
mod repositories;

use std::env;

type AllResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> AllResult<()> {
    dotenvy::dotenv()?;
    let url = env::var("DATABASE_URL")?;
    let connection = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!().run(&connection).await?;
    use models::BetStatus;
    let result = sqlx::query!(
        r#"
        SELECT
            bet_id, user_id, for_bet, bet_amount, participants.paid_out AS participant_paid,
            id, creator_id, description, status AS "status: BetStatus", stop_bets_at, created_at, updated_at, bets.paid_out, paid_out_at
        FROM bet_participants AS participants JOIN bets ON bet_id = id WHERE user_id = $1;
        "#,
        2
    ).map(|row| (
        models::Bet {
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
        models::BetParticipant {
            bet_id: row.bet_id,
            user_id: row.user_id,
            for_bet: row.for_bet,
            bet_amount: row.bet_amount,
            paid_out: row.participant_paid,
        },
    ))
        .fetch_all(&connection)
        .await?;

    println!("{:?}", result);

    Ok(())
}

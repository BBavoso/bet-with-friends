use super::repositories::bet_participants;
use crate::AllResult;
use sqlx::PgPool;

#[derive(Debug)]
pub struct BetParticipant {
    pub bet_id: i32,
    pub user_id: i32,
    pub for_bet: bool,
    pub bet_amount: i32,
    pub paid_out: bool,
}

impl BetParticipant {
    async fn read_from_id(connection: &PgPool, id: i32) -> AllResult<BetParticipant> {
        bet_participants::get_bet_participant_by_bet_id(connection, id).await
    }
}

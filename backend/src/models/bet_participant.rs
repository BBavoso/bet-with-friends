use super::{
    repositories::{bet_participants, users},
    User,
};
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
    pub async fn read_from_id(connection: &PgPool, id: i32) -> AllResult<Self> {
        bet_participants::get_bet_participant_by_bet_id(connection, id).await
    }

    pub async fn user(&self, connection: &PgPool) -> AllResult<User> {
        users::read_user_with_id(connection, self.user_id).await
    }
}

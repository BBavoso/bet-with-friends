use super::{
    repositories::{bet_participants, bets},
    BetParticipant,
};
use crate::AllResult;
use sqlx::{types::chrono::NaiveDateTime, PgPool};

#[derive(sqlx::Type, PartialEq, Debug, Clone, Copy)]
#[sqlx(type_name = "bet_status", rename_all = "lowercase")]
pub enum BetStatus {
    Active,
    Finished,
    #[sqlx(rename = "payed_out")]
    PayedOut,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bet {
    pub id: i32,
    pub creator_id: i32,
    pub description: String,
    pub status: BetStatus,
    pub stop_bets_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub paid_out: bool,
    pub paid_out_at: Option<NaiveDateTime>,
}

impl Bet {
    pub async fn read_by_id(connection: &PgPool, id: i32) -> AllResult<Bet> {
        bets::get_bet_by_id(connection, id).await
    }

    pub async fn read_all_by_status(
        connection: &PgPool,
        status: &BetStatus,
    ) -> AllResult<Vec<Bet>> {
        bets::get_bets_by_status(connection, status).await
    }

    pub async fn close(&mut self, connection: &PgPool) -> AllResult<()> {
        bets::close_bet(connection, self).await
    }

    pub async fn payout(&mut self, connection: &PgPool, bet_outcome: bool) -> AllResult<()> {
        bets::payout_bet(connection, self, bet_outcome).await
    }

    pub async fn participants(&self, connection: &PgPool) -> AllResult<Vec<BetParticipant>> {
        bet_participants::get_bet_participants(connection, self).await
    }
}

use sqlx::types::chrono::NaiveDateTime;

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

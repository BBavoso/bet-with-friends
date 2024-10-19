#[derive(Debug)]
pub struct BetParticipant {
    pub bet_id: i32,
    pub user_id: i32,
    pub for_bet: bool,
    pub bet_amount: i32,
    pub paid_out: bool,
}

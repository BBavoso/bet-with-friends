#[derive(Debug, PartialEq)]
pub struct Score {
    pub user_id: i32,
    pub total_wins: i32,
    pub total_losses: i32,
    pub points_earned: i32,
}

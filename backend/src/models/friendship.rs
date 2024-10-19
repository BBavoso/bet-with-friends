use sqlx::types::chrono::NaiveDateTime;

#[derive(sqlx::Type, PartialEq, Debug)]
#[sqlx(type_name = "friendship_status", rename_all = "lowercase")]
pub enum FriendshipStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, PartialEq)]
pub struct Friendship {
    pub user_id: i32,
    pub friend_id: i32,
    pub status: FriendshipStatus,
    pub created_at: NaiveDateTime,
}

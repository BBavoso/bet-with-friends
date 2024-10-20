use super::{repositories::friendships, User};
use crate::AllResult;
use sqlx::{types::chrono::NaiveDateTime, PgPool};

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

impl Friendship {
    async fn read_from_users(
        connection: &PgPool,
        sender: &User,
        recipient: &User,
    ) -> AllResult<Friendship> {
        friendships::get_friendship(connection, sender, recipient).await
    }
}

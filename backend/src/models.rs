#![allow(dead_code)]

use sqlx::{prelude::FromRow, types::chrono};

#[derive(FromRow, Debug)]
pub(crate) struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "friendship_status", rename_all = "lowercase")]
enum FriendshipStatus {
    Pending,
    Accepted,
    Rejected,
}

pub(crate) struct Friendship {
    user_id: i32,
    friend_id: i32,
    status: FriendshipStatus,
    created_at: chrono::NaiveDateTime,
}

#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
enum BetStatus {
    #[sqlx(rename = "not_started")]
    NotStarted,
    Active,
    Finished,
    #[sqlx(rename = "payed_out")]
    PayedOut,
}

pub(crate) struct Bet {
    id: i32,
    creator_id: i32,
    description: String,
    bet_amount: i32,
    status: BetStatus,
    start_time: chrono::NaiveDateTime,
    end_time: chrono::NaiveDateTime,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    paid_out: bool,
    paid_out_at: chrono::NaiveDateTime,
}

pub(crate) struct Score {
    user_id: i32,
    total_wins: i32,
    total_losses: i32,
    points_earned: i32,
}

pub(crate) struct BetParticipant {
    bet_id: i32,
    user_id: i32,
    is_winner: bool,
}

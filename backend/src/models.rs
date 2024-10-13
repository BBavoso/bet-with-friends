#![allow(dead_code)]

use sqlx::{prelude::FromRow, types::chrono};

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

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
    pub created_at: chrono::NaiveDateTime,
}

#[derive(sqlx::Type, PartialEq, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum BetStatus {
    #[sqlx(rename = "not_started")]
    NotStarted,
    Active,
    Finished,
    #[sqlx(rename = "payed_out")]
    PayedOut,
}

pub struct Bet {
    pub id: i32,
    pub creator_id: i32,
    pub description: String,
    pub bet_amount: i32,
    pub status: BetStatus,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub paid_out: bool,
    pub paid_out_at: chrono::NaiveDateTime,
}

#[derive(Debug, PartialEq)]
pub struct Score {
    pub user_id: i32,
    pub total_wins: i32,
    pub total_losses: i32,
    pub points_earned: i32,
}

pub struct BetParticipant {
    pub bet_id: i32,
    pub user_id: i32,
    pub is_winner: bool,
}

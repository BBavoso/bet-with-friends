use sqlx::{prelude::FromRow, types::chrono::NaiveDateTime};

#[derive(FromRow, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#![allow(dead_code)]

pub mod users {
    use crate::{models::User, AllResult};

    pub async fn get_user_with_id(connection: &sqlx::PgPool, id: i32) -> AllResult<Vec<User>> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id);
        let users = query.fetch_all(connection).await?;
        Ok(users)
    }

    pub async fn get_user_with_username(
        connection: &sqlx::PgPool,
        username: &str,
    ) -> AllResult<Vec<User>> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username);
        let users = query.fetch_all(connection).await?;
        Ok(users)
    }

    pub async fn create_user(
        connection: &sqlx::PgPool,
        username: String,
        email: String,
        password_hash: String,
    ) -> AllResult<User> {
        let query = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING *;",
            username,
            email,
            password_hash
        );
        let user = query.fetch_one(connection).await?;
        Ok(user)
    }
}

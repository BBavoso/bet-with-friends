#![allow(dead_code)]

use crate::{models::User, AllResult};

pub async fn get_user_with_id(connection: &sqlx::PgPool, id: i32) -> AllResult<User> {
    let query = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id);
    let user = query.fetch_one(connection).await?;
    Ok(user)
}

pub async fn get_user_with_username(connection: &sqlx::PgPool, username: &str) -> AllResult<User> {
    let query = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username);
    let user = query.fetch_one(connection).await?;
    Ok(user)
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_user(pool: PgPool) -> AllResult<()> {
        let user = create_user(
            &pool,
            "john".into(),
            "john@mail.com".into(),
            "pass_ABCD".into(),
        )
        .await?;
        assert_eq!(user.username, "john");
        assert_eq!(user.email, "john@mail.com");
        Ok(())
    }

    #[sqlx::test]
    async fn test_user_by_id(pool: PgPool) -> AllResult<()> {
        create_user(
            &pool,
            "john".into(),
            "john@mail.com".into(),
            "pass_ABCD".into(),
        )
        .await?;
        let user = get_user_with_id(&pool, 1).await?;
        assert_eq!(user.username, "john");
        assert_eq!(user.email, "john@mail.com");
        Ok(())
    }

    #[sqlx::test]
    async fn test_user_by_username(pool: PgPool) -> AllResult<()> {
        create_user(
            &pool,
            "john".into(),
            "john@mail.com".into(),
            "pass_ABCD".into(),
        )
        .await?;
        let user = get_user_with_username(&pool, "john").await?;
        assert_eq!(user.username, "john");
        assert_eq!(user.email, "john@mail.com");
        Ok(())
    }
}

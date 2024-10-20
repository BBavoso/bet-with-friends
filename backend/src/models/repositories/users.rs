use crate::{models::User, AllResult};

use super::scores::create_default_score;

pub async fn read_user_with_id(connection: &sqlx::PgPool, id: i32) -> AllResult<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE id = $1
        "#,
        id
    )
    .fetch_one(connection)
    .await?;
    Ok(user)
}

pub async fn read_user_with_username(connection: &sqlx::PgPool, username: &str) -> AllResult<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE username = $1
        "#,
        username
    )
    .fetch_one(connection)
    .await?;
    Ok(user)
}

pub async fn create_user(
    connection: &sqlx::PgPool,
    username: String,
    email: String,
    password_hash: String,
) -> AllResult<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING *;
        "#,
        username,
        email,
        password_hash
    )
    .fetch_one(connection)
    .await?;
    create_default_score(connection, &user).await?;
    Ok(user)
}

#[cfg(test)]
pub async fn create_users<T>(pool: &sqlx::PgPool, usernames: Vec<T>) -> AllResult<Vec<User>>
where
    T: Into<String> + Clone,
{
    let mut users = Vec::with_capacity(usernames.len());
    for username in usernames {
        let user = create_user(
            &pool,
            username.clone().into(),
            username.into() + "@mail.com",
            "pass123".into(),
        )
        .await?;
        users.push(user);
    }
    users.reverse();
    Ok(users)
}

#[cfg(test)]
mod unit_tests {
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
        let user = read_user_with_id(&pool, 1).await?;
        assert_eq!(user.username, "john");
        assert_eq!(user.email, "john@mail.com");
        Ok(())
    }

    #[sqlx::test]
    async fn test_user_by_username(pool: PgPool) -> AllResult<()> {
        let created_user = create_user(
            &pool,
            "john".into(),
            "john@mail.com".into(),
            "pass_ABCD".into(),
        )
        .await?;
        let read_user = read_user_with_username(&pool, "john").await?;
        assert_eq!(read_user, created_user);
        Ok(())
    }
}

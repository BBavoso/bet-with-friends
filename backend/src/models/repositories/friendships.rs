use crate::models::{Friendship, FriendshipStatus, User};
use crate::AllResult;

pub async fn get_friendship(
    connection: &sqlx::PgPool,
    sender: &User,
    recipient: &User,
) -> AllResult<Friendship> {
    let friendship = sqlx::query_as!(
        Friendship,
        r#"
        SELECT user_id, friend_id, status AS "status: FriendshipStatus", created_at
        FROM friendships WHERE user_id = $1 AND friend_id = $2
        "#,
        sender.id,
        recipient.id
    )
    .fetch_one(connection)
    .await?;
    Ok(friendship)
}

pub async fn get_friendships(connection: &sqlx::PgPool, user: &User) -> AllResult<Vec<Friendship>> {
    let friendships = sqlx::query_as!(
        Friendship,
        r#"
        SELECT user_id, friend_id, status AS "status: FriendshipStatus", created_at
        FROM friendships WHERE user_id = $1
        "#,
        user.id,
    )
    .fetch_all(connection)
    .await?;
    Ok(friendships)
}

pub async fn send_friend_request(
    connection: &sqlx::PgPool,
    sender: &User,
    recipient: &User,
) -> AllResult<Friendship> {
    let friendship = sqlx::query_as!(
        Friendship,
        r#"
        INSERT INTO friendships (user_id, friend_id, status)
        VALUES ($1, $2, $3)
        RETURNING user_id, friend_id, status AS "status: FriendshipStatus", created_at
        "#,
        sender.id,
        recipient.id,
        FriendshipStatus::Pending as _,
    )
    .fetch_one(connection)
    .await?;
    Ok(friendship)
}

async fn create_friendship(
    connection: &sqlx::PgPool,
    user_1: &User,
    user_2: &User,
    status: FriendshipStatus,
) -> AllResult<Friendship> {
    let now = sqlx::types::chrono::Local::now().naive_local();
    let friendship = sqlx::query_as!(
        Friendship,
        r#"
        INSERT INTO friendships (user_id, friend_id, status, responded_at)
        VALUES ($1, $2, $3, $4)
        RETURNING user_id, friend_id, status AS "status: FriendshipStatus", created_at
        "#,
        user_1.id,
        user_2.id,
        status as _,
        now
    )
    .fetch_one(connection)
    .await?;
    Ok(friendship)
}

#[derive(Clone, Copy)]
pub enum FriendRequestResponse {
    Accept,
    Reject,
}

impl From<FriendRequestResponse> for FriendshipStatus {
    fn from(response: FriendRequestResponse) -> Self {
        match response {
            FriendRequestResponse::Accept => FriendshipStatus::Accepted,
            FriendRequestResponse::Reject => FriendshipStatus::Rejected,
        }
    }
}

pub async fn respond_to_friend_request(
    connection: &sqlx::PgPool,
    user: &User,
    responding_to: &User,
    response: FriendRequestResponse,
) -> AllResult<(Friendship, Option<Friendship>)> {
    let friendship = get_friendship(connection, responding_to, user).await?;
    assert_eq!(friendship.status, FriendshipStatus::Pending);

    let new_status: FriendshipStatus = response.into();
    let now = sqlx::types::chrono::Local::now().naive_local();
    let response_friendship = sqlx::query_as!(
        Friendship,
        r#"
        UPDATE friendships
        SET status = $1, responded_at = $2
        WHERE user_id = $3 AND friend_id = $4
        RETURNING user_id, friend_id, status AS "status: FriendshipStatus", created_at
        "#,
        new_status as _,
        now,
        responding_to.id,
        user.id,
    )
    .fetch_one(connection)
    .await?;

    let new_friendship = match response {
        FriendRequestResponse::Accept => Some(
            create_friendship(connection, user, responding_to, FriendshipStatus::Accepted).await?,
        ),
        FriendRequestResponse::Reject => None,
    };

    Ok((response_friendship, new_friendship))
}

#[cfg(test)]
mod tests {
    use super::super::users::create_users;
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_send_friend_request(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob", "John"]).await?;
        let bob = users.pop().unwrap();
        let john = users.pop().unwrap();

        let friendship = send_friend_request(&pool, &john, &bob).await?;

        assert_eq!(friendship.user_id, john.id);
        assert_eq!(friendship.friend_id, bob.id);
        assert_eq!(friendship.status, FriendshipStatus::Pending);
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_friendship(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob", "John"]).await?;
        let bob = users.pop().unwrap();
        let john = users.pop().unwrap();

        send_friend_request(&pool, &john, &bob).await?;

        let friendship = get_friendship(&pool, &john, &bob).await?;

        assert_eq!(friendship.user_id, john.id);
        assert_eq!(friendship.friend_id, bob.id);
        assert_eq!(friendship.status, FriendshipStatus::Pending);
        Ok(())
    }

    #[sqlx::test]
    async fn test_get_friendships(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob", "John", "Mark"]).await?;
        let bob = users.pop().unwrap();
        let john = users.pop().unwrap();
        let mark = users.pop().unwrap();

        send_friend_request(&pool, &john, &bob).await?;
        send_friend_request(&pool, &mark, &john).await?;
        send_friend_request(&pool, &mark, &bob).await?;

        let john_friendships = get_friendships(&pool, &john).await?;
        assert_eq!(john_friendships.len(), 1);

        let mark_friendships = get_friendships(&pool, &mark).await?;
        assert_eq!(mark_friendships.len(), 2);

        Ok(())
    }

    #[sqlx::test]
    async fn accept_friend_request(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob", "John"]).await?;
        let bob = users.pop().unwrap();
        let john = users.pop().unwrap();

        let friend_request = send_friend_request(&pool, &john, &bob).await?;

        assert_eq!(friend_request.user_id, john.id);
        assert_eq!(friend_request.friend_id, bob.id);
        assert_eq!(friend_request.status, FriendshipStatus::Pending);

        let response =
            respond_to_friend_request(&pool, &bob, &john, FriendRequestResponse::Accept).await?;

        let response_friendship_1 = response.0;
        assert_eq!(response_friendship_1.user_id, john.id);
        assert_eq!(response_friendship_1.friend_id, bob.id);
        assert_eq!(response_friendship_1.status, FriendshipStatus::Accepted);

        let response_friendship_2 = response.1;
        assert!(response_friendship_2.is_some());
        let response_friendship_2 = response_friendship_2.unwrap();

        assert_eq!(response_friendship_2.user_id, bob.id);
        assert_eq!(response_friendship_2.friend_id, john.id);
        assert_eq!(response_friendship_2.status, FriendshipStatus::Accepted);

        let friendship_1 = get_friendship(&pool, &john, &bob).await?;
        assert_eq!(friendship_1.user_id, john.id);
        assert_eq!(friendship_1.friend_id, bob.id);
        assert_eq!(friendship_1.status, FriendshipStatus::Accepted);

        let friendship_2 = get_friendship(&pool, &bob, &john).await?;
        assert_eq!(friendship_2.user_id, bob.id);
        assert_eq!(friendship_2.friend_id, john.id);
        assert_eq!(friendship_2.status, FriendshipStatus::Accepted);

        Ok(())
    }

    #[sqlx::test]
    async fn reject_friend_request(pool: PgPool) -> AllResult<()> {
        let mut users = create_users(&pool, vec!["Bob", "John"]).await?;
        let bob = users.pop().unwrap();
        let john = users.pop().unwrap();

        let request = send_friend_request(&pool, &john, &bob).await?;

        assert_eq!(request.user_id, john.id);
        assert_eq!(request.friend_id, bob.id);
        assert_eq!(request.status, FriendshipStatus::Pending);

        let response =
            respond_to_friend_request(&pool, &bob, &john, FriendRequestResponse::Reject).await?;
        let response_1 = response.0;
        let response_2 = response.1;

        assert_eq!(response_1.user_id, john.id);
        assert_eq!(response_1.friend_id, bob.id);
        assert_eq!(response_1.status, FriendshipStatus::Rejected);

        assert!(response_2.is_none());

        let friendship_1 = get_friendship(&pool, &john, &bob).await?;
        assert_eq!(friendship_1.user_id, john.id);
        assert_eq!(friendship_1.friend_id, bob.id);
        assert_eq!(friendship_1.status, FriendshipStatus::Rejected);

        let friendship_2 = get_friendship(&pool, &bob, &john).await;
        assert!(friendship_2.is_err());

        Ok(())
    }
}

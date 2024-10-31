use super::{
    repositories::{
        bet_participants, bets,
        friendships::{self, FriendRequestResponse},
        scores, users,
    },
    Bet, BetParticipant, Friendship, Score,
};
use crate::AllResult;
use serde::Serialize;
use sqlx::{prelude::FromRow, types::chrono::NaiveDateTime, PgPool};

#[derive(FromRow, Debug, PartialEq, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub async fn new(
        connection: &PgPool,
        username: String,
        email: String,
        password: String,
    ) -> AllResult<Self> {
        let password_hash = hash_password(password);
        users::create_user(connection, username, email, password_hash).await
    }

    pub async fn read_from_id(connection: &PgPool, id: i32) -> AllResult<Self> {
        users::read_user_with_id(connection, id).await
    }

    pub async fn read_from_name(connection: &PgPool, username: &str) -> AllResult<Self> {
        users::read_user_with_username(connection, username).await
    }

    pub async fn create_default_score(&self, connection: &PgPool) -> AllResult<Score> {
        scores::create_default_score(connection, self).await
    }

    pub async fn score(&self, connection: &PgPool) -> AllResult<Score> {
        scores::read_user_score(connection, self).await
    }

    pub async fn friendships_all(&self, connection: &PgPool) -> AllResult<Vec<Friendship>> {
        friendships::get_all_friendships(connection, self).await
    }

    pub async fn friendships_accepted(&self, connection: &PgPool) -> AllResult<Vec<Friendship>> {
        friendships::get_accepted_friendships(connection, self).await
    }

    pub async fn send_friend_request(
        &self,
        connection: &PgPool,
        to_user: &User,
    ) -> AllResult<Friendship> {
        friendships::send_friend_request(connection, self, to_user).await
    }

    pub async fn accept_friend_request(
        &self,
        connection: &PgPool,
        responding_to: &User,
    ) -> AllResult<()> {
        friendships::respond_to_friend_request(
            connection,
            self,
            responding_to,
            FriendRequestResponse::Accept,
        )
        .await?;
        Ok(())
    }

    pub async fn reject_friend_request(
        &self,
        connection: &PgPool,
        responding_to: &User,
    ) -> AllResult<()> {
        friendships::respond_to_friend_request(
            connection,
            self,
            responding_to,
            FriendRequestResponse::Reject,
        )
        .await?;
        Ok(())
    }

    pub async fn bets_created(&self, connection: &PgPool) -> AllResult<Vec<Bet>> {
        bets::get_bets_by_user(connection, self).await
    }

    pub async fn create_timeless_bet(
        &self,
        connection: &PgPool,
        description: String,
    ) -> AllResult<Bet> {
        bets::create_timeless_bet(connection, self, description).await
    }

    pub async fn create_timed_bet(
        &self,
        connection: &PgPool,
        description: String,
        stop_bets_at: NaiveDateTime,
    ) -> AllResult<Bet> {
        bets::create_timed_bet(connection, self, description, stop_bets_at).await
    }

    pub async fn bets(&self, connection: &PgPool) -> AllResult<Vec<BetParticipant>> {
        bet_participants::get_bet_participants_by_bet_user(connection, self).await
    }

    pub async fn particpate_in_bet(
        &self,
        connection: &PgPool,
        bet: &Bet,
        amount: i32,
        for_bet: bool,
    ) -> AllResult<BetParticipant> {
        bet_participants::create_bet_participant(connection, self, bet, amount, for_bet).await
    }
}

fn hash_password(password: String) -> String {
    // TODO: HASH function
    password
}

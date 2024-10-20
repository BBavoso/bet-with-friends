use crate::AllResult;

use super::{
    repositories::{
        bet_participants, bets,
        friendships::{self, FriendRequestResponse},
        scores, users,
    },
    Bet, BetParticipant, Friendship, Score,
};
use sqlx::{prelude::FromRow, types::chrono::NaiveDateTime, PgPool};

#[derive(FromRow, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    async fn new(
        connection: &PgPool,
        username: String,
        email: String,
        password_hash: String,
    ) -> AllResult<Self> {
        users::create_user(connection, username, email, password_hash).await
    }

    async fn read_from_id(connection: &PgPool, id: i32) -> AllResult<Self> {
        users::read_user_with_id(connection, id).await
    }

    async fn read_from_name(connection: &PgPool, username: &str) -> AllResult<Self> {
        users::read_user_with_username(connection, username).await
    }

    async fn create_default_score(&self, connection: &PgPool) -> AllResult<Score> {
        scores::create_default_score(connection, self).await
    }

    async fn score(&self, connection: &PgPool) -> AllResult<Score> {
        scores::read_user_score(connection, self).await
    }

    async fn friendships(&self, connection: &PgPool) -> AllResult<Vec<Friendship>> {
        friendships::get_friendships(connection, self).await
    }

    async fn send_friend_request(
        &self,
        connection: &PgPool,
        to_user: &User,
    ) -> AllResult<Friendship> {
        friendships::send_friend_request(connection, self, to_user).await
    }

    async fn accept_friend_request(
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

    async fn reject_friend_request(
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

    async fn bets_created(&self, connection: &PgPool) -> AllResult<Vec<Bet>> {
        bets::get_bets_by_user(connection, self).await
    }

    async fn create_timeless_bet(
        &self,
        connection: &PgPool,
        description: String,
    ) -> AllResult<Bet> {
        bets::create_timeless_bet(connection, self, description).await
    }

    async fn create_timed_bet(
        &self,
        connection: &PgPool,
        description: String,
        stop_bets_at: NaiveDateTime,
    ) -> AllResult<Bet> {
        bets::create_timed_bet(connection, self, description, stop_bets_at).await
    }

    async fn bets(&self, connection: &PgPool) -> AllResult<Vec<BetParticipant>> {
        bet_participants::get_bet_participants_by_bet_user(connection, self).await
    }

    async fn particpate_in_bet(
        &self,
        connection: &PgPool,
        bet: &Bet,
        amount: i32,
        for_bet: bool,
    ) -> AllResult<BetParticipant> {
        bet_participants::create_bet_participant(connection, self, bet, amount, for_bet).await
    }
}

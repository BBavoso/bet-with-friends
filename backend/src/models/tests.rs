use sqlx::PgPool;

use crate::AllResult;

use super::*;

#[sqlx::test]
fn run_bet(pool: PgPool) -> AllResult<()> {
    let user1 = User::new(
        &pool,
        "user1".into(),
        "user1@mail.com".into(),
        "user1pass".into(),
    )
    .await?;

    let user2 = User::new(
        &pool,
        "user2".into(),
        "user2@mail.com".into(),
        "user2pass".into(),
    )
    .await?;

    let user3 = User::new(
        &pool,
        "user3".into(),
        "user3@mail.com".into(),
        "user3pass".into(),
    )
    .await?;

    assert_ne!(user1.id, user2.id);
    assert_ne!(user1.id, user3.id);
    assert_ne!(user2.id, user3.id);

    assert_eq!(user1.username, "user1");
    assert_eq!(user2.username, "user2");
    assert_eq!(user3.username, "user3");

    let user1_score = user1.score(&pool).await?;

    assert_eq!(user1_score.points_earned, 0);
    assert_eq!(user1_score.total_wins, 0);
    assert_eq!(user1_score.total_losses, 0);
    assert_eq!(user1_score.user_id, user1.id);

    let mut bet1 = user1.create_timeless_bet(&pool, "bet1".into()).await?;
    let bet2 = user1.create_timeless_bet(&pool, "bet1".into()).await?;
    let bet3 = user2.create_timeless_bet(&pool, "bet1".into()).await?;
    let bet4 = user3.create_timeless_bet(&pool, "bet1".into()).await?;

    assert_eq!(user1.bets_created(&pool).await?.len(), 2);
    assert_eq!(user2.bets_created(&pool).await?.len(), 1);
    assert_eq!(user3.bets_created(&pool).await?.len(), 1);

    assert!(user1.bets_created(&pool).await?.contains(&bet1));
    assert!(user1.bets_created(&pool).await?.contains(&bet2));
    assert!(user2.bets_created(&pool).await?.contains(&bet3));
    assert!(user3.bets_created(&pool).await?.contains(&bet4));

    user1.particpate_in_bet(&pool, &bet1, 10, true).await?;
    user2.particpate_in_bet(&pool, &bet1, 20, false).await?;
    user3.particpate_in_bet(&pool, &bet1, 30, true).await?;

    let bet1_participants = bet1.participants(&pool).await?;
    assert_eq!(bet1_participants.len(), 3);

    assert_eq!(bet1.status, BetStatus::Active);

    bet1.close(&pool).await?;

    assert_eq!(bet1.status, BetStatus::Finished);

    bet1.payout(&pool, true).await?;

    assert_eq!(bet1.status, BetStatus::PayedOut);

    assert_eq!(user1.score(&pool).await?.points_earned, 10);
    assert_eq!(user1.score(&pool).await?.total_wins, 1);
    assert_eq!(user1.score(&pool).await?.total_losses, 0);

    assert_eq!(user2.score(&pool).await?.points_earned, 0);
    assert_eq!(user2.score(&pool).await?.total_wins, 0);
    assert_eq!(user2.score(&pool).await?.total_losses, 1);

    assert_eq!(user3.score(&pool).await?.points_earned, 30);
    assert_eq!(user3.score(&pool).await?.total_wins, 1);
    assert_eq!(user3.score(&pool).await?.total_losses, 0);

    let particpants = bet1.participants(&pool).await?;

    assert!(particpants.iter().all(|participant| participant.paid_out));

    Ok(())
}

#[sqlx::test]
fn friendship(pool: PgPool) -> AllResult<()> {
    let user1 = User::new(
        &pool,
        "user1".into(),
        "user1@mail.com".into(),
        "user1pass".into(),
    )
    .await?;

    let user2 = User::new(
        &pool,
        "user2".into(),
        "user2@mail.com".into(),
        "user2pass".into(),
    )
    .await?;

    let user3 = User::new(
        &pool,
        "user3".into(),
        "user3@mail.com".into(),
        "user3pass".into(),
    )
    .await?;

    let friend_request = user1.send_friend_request(&pool, &user2).await?;
    assert_eq!(friend_request.user_id, user1.id);
    assert_eq!(friend_request.friend_id, user2.id);
    assert_eq!(friend_request.status, FriendshipStatus::Pending);

    user1.send_friend_request(&pool, &user3).await?;
    user2.send_friend_request(&pool, &user3).await?;

    user2.accept_friend_request(&pool, &user1).await?;
    user3.accept_friend_request(&pool, &user2).await?;
    user3.reject_friend_request(&pool, &user1).await?;

    let user1_friends = user1.friendships_accepted(&pool).await?;
    let user1_friendships = user1.friendships_all(&pool).await?;

    assert_eq!(user1_friends.len(), 1);
    assert_eq!(user1_friendships.len(), 2);

    assert_eq!(user2.friendships_accepted(&pool).await?.len(), 2);

    Ok(())
}

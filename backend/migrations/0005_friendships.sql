CREATE TABLE "friendships" (
  "user_id" INTEGER NOT NULL,
  "friend_id" INTEGER NOT NULL,
  "status" friendship_status NOT NULL DEFAULT 'pending',
  "created_at" TIMESTAMP NOT NULL DEFAULT (NOW()),
  "responded_at" TIMESTAMP DEFAULT NULL,
  PRIMARY KEY ("user_id", "friend_id")
);
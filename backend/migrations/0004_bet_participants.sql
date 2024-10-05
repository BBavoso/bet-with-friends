CREATE TABLE "bet_participants" (
  "bet_id" INTEGER,
  "user_id" INTEGER,
  "is_winner" BOOLEAN NOT NULL,
  PRIMARY KEY ("bet_id", "user_id")
);
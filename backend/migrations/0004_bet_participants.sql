CREATE TABLE "bet_participants" (
  "bet_id" INTEGER,
  "user_id" INTEGER,
  "is_winner" BOOLEAN NOT NULL,
  "bet_amount" INTEGER NOT NULL,
  "for_bet" BOOLEAN NOT NULL,
  PRIMARY KEY ("bet_id", "user_id")
);
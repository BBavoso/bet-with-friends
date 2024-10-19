CREATE TABLE "bet_participants" (
  "bet_id" INTEGER,
  "user_id" INTEGER,
  "for_bet" BOOLEAN NOT NULL,
  "bet_amount" INTEGER NOT NULL,
  "paid_out" BOOLEAN NOT NULL,
  PRIMARY KEY ("bet_id", "user_id")
);
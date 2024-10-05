CREATE TABLE "bets" (
  "id" SERIAL PRIMARY KEY,
  "creator_id" INTEGER NOT NULL,
  "description" TEXT NOT NULL,
  "bet_amount" INTEGER NOT NULL,
  "status" bet_status NOT NULL,
  "start_time" TIMESTAMP,
  "end_time" TIMESTAMP,
  "created_at" TIMESTAMP NOT NULL DEFAULT (NOW()),
  "updated_at" TIMESTAMP NOT NULL DEFAULT (NOW()),
  "paid_out" BOOLEAN NOT NULL,
  "paid_out_at" TIMESTAMP
);
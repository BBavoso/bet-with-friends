ALTER TABLE "bets" ADD FOREIGN KEY ("creator_id") REFERENCES "users" ("id");

ALTER TABLE "bet_participants" ADD FOREIGN KEY ("bet_id") REFERENCES "bets" ("id");

ALTER TABLE "bet_participants" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "friendships" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "friendships" ADD FOREIGN KEY ("friend_id") REFERENCES "users" ("id");

ALTER TABLE "scores" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

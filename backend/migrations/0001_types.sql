CREATE TYPE "bet_status" AS ENUM (
  'active',
  'finished',
  'payed_out'
);

CREATE TYPE "friendship_status" AS ENUM (
  'pending',
  'accepted',
  'rejected'
);
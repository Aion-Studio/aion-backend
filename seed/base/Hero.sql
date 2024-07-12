-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-06-27 8:52:52.8890 AM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."Class";
CREATE TYPE "public"."Class" AS ENUM ('Fighter', 'Ranger', 'Wizard');

-- Table Definition
CREATE TABLE "public"."Hero" (
    "id" text NOT NULL,
    "class" "public"."Class" NOT NULL,
    "hp" int4 NOT NULL,
    "strength" int4 NOT NULL,
    "dexterity" int4 NOT NULL,
    "intelligence" int4 NOT NULL,
    "explore" int4 NOT NULL,
    "crafting" int4 NOT NULL,
    "accountId" text,
    "level" int4 NOT NULL,
    "armor" int4 NOT NULL,
    "name" text NOT NULL,
    CONSTRAINT "Hero_accountId_fkey" FOREIGN KEY ("accountId") REFERENCES "public"."Account"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Hero" ("id", "class", "hp", "strength", "dexterity", "intelligence", "explore", "crafting", "accountId", "level", "armor", "name") VALUES
('2a038e2f-4de8-4907-b48a-c5ce89f876e3', 'Fighter', 89, 5, 2, 1, 15, 20, 'f42dc222-0dd2-43c4-b581-c36ded16bac5', 1, 1, 'ElFal the Fierce'),
('90e64955-1208-4e8d-ab5e-04b3dbe86b6b', 'Wizard', 77, 2, 2, 0, 15, 20, 'c081e311-abe2-49f6-a48d-26dc4dbd1f53', 1, 1, 'VorZaDra'),
('a50ce823-b66c-4873-bf4e-1b38b45f8e5e', 'Wizard', 92, 4, 1, 2, 15, 20, 'b6ee7843-bbd9-4937-8272-5c633e03b880', 1, 1, 'FalHelRo');

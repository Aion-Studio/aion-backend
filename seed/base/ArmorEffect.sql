-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-05-16 7:31:53.4500 AM
-- -------------------------------------------------------------


DROP TABLE IF EXISTS "public"."ArmorEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."ArmorEffect" (
    "id" text NOT NULL,
    "amount" int4 NOT NULL,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."ArmorEffect" ("id", "amount") VALUES
('0b9964d7-a433-4a32-a2f6-7d0f652ba525', -10),
('7a83aac5-759b-45d2-ac50-3ef5bb46b034', -1),
('a7f88339-ed50-4d14-a75f-755fa67014a8', 2);

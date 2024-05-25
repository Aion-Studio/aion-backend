-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-05-16 8:26:39.2910 AM
-- -------------------------------------------------------------


DROP TABLE IF EXISTS "public"."BaseStats";
-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."BaseStats" (
    "id" text NOT NULL,
    "level" int4 NOT NULL,
    "xp" int4 NOT NULL,
    "damageMin" int4 NOT NULL,
    "damageMax" int4 NOT NULL,
    "hit_points" int4 NOT NULL,
    "armor" int4 NOT NULL,
    "resilience" int4 NOT NULL DEFAULT 0,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."BaseStats" ("id", "level", "xp", "damageMin", "damageMax", "hit_points", "armor", "resilience") VALUES
('1f3c451b-ff41-45f4-a237-27f7b8cadbe0', 1, 0, 24, 35, 626, 3, 0),
('92f1cd1d-a23d-4e07-ab04-be7987d589c8', 1, 0, 4, 6, 91, 8, 0),
('9e730d95-79b6-44a6-801a-217bbd503dd5', 1, 0, 4, 6, 102, 5, 0),
('cf496a19-39bd-49e7-af1b-eba156d02921', 1, 85, 24, 43, 30, 3, 0),
('d0bd8ea1-29a4-4cc5-840a-26f4d6120c02', 1, 0, 4, 6, 97, 14, 0);

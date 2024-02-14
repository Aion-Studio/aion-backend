-- -------------------------------------------------------------
-- TablePlus 5.8.6(534)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-02-12 10:30:03.4910 AM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Npc" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "level" int4 NOT NULL,
    "resilience" int4 NOT NULL,
    "strength" int4 NOT NULL,
    "agility" int4 NOT NULL,
    "hp" int4 NOT NULL DEFAULT 150,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Npc" ("id", "name", "level", "resilience", "strength", "agility", "hp") VALUES
('ef5666e9-95ee-4097-a643-afef984efee5', 'Ragnor', 1, 2, 5, 2, 150);

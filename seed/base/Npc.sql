-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-05-16 8:28:44.4290 AM
-- -------------------------------------------------------------

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS "public"."Npc";
-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Npc" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "level" int4 NOT NULL,
    "resilience" int4 NOT NULL,
    "strength" int4 NOT NULL,
    "agility" int4 NOT NULL,
    "hp" int4 NOT NULL DEFAULT 150,
    "damageMax" int4 NOT NULL DEFAULT 0,
    "damageMin" int4 NOT NULL DEFAULT 0,
    "armor" int4 NOT NULL DEFAULT 0,
    "deckId" text,
    CONSTRAINT "Npc_deckId_fkey" FOREIGN KEY ("deckId") REFERENCES "public"."Deck"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

-- Indices
CREATE UNIQUE INDEX "Npc_deckId_key" ON public."Npc" USING btree ("deckId");

-- Insert into Deck table and get the new deckId
WITH new_deck AS (
    INSERT INTO "public"."Deck" (id /*, other columns for Deck table */)
    VALUES (uuid_generate_v4() /*, values for other columns */)
    RETURNING "id"
),
new_npc AS (
    -- Insert into Npc table using the new deckId and get the new npcId
    INSERT INTO "public"."Npc" ("id", "name", "level", "resilience", "strength", "agility", "hp", "damageMax", "damageMin", "armor", "deckId")
    SELECT uuid_generate_v4(), 'Ragnor', 1, 2, 5, 2, 30, 14, 10, 2, id
    FROM new_deck
    RETURNING "id"
)
-- Insert into Action table using the new npcId
INSERT INTO "public"."Action" ("id", "name", "region_name", "leyline_id", "quest_id", "description", "cost", "npc_id")
SELECT uuid_generate_v4(), 'FightNpc', 'Dusane', NULL, (SELECT id from "Quest" LIMIT 1), 'ActionDescription', 0, id
FROM new_npc;
-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-05-21 8:02:09.6700 AM
-- -------------------------------------------------------------


DROP TABLE IF EXISTS "public"."Action";
-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Action" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "region_name" text NOT NULL,
    "leyline_id" text,
    "quest_id" text,
    "description" text,
    "cost" int4 DEFAULT 0,
    "npc_id" text,
    CONSTRAINT "Action_leyline_id_fkey" FOREIGN KEY ("leyline_id") REFERENCES "public"."Leyline"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "Action_quest_id_fkey" FOREIGN KEY ("quest_id") REFERENCES "public"."Quest"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "Action_region_name_fkey" FOREIGN KEY ("region_name") REFERENCES "public"."Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "Action_npc_id_fkey" FOREIGN KEY ("npc_id") REFERENCES "public"."Npc"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

-- Indices
CREATE UNIQUE INDEX "Action_name_description_quest_id_key" ON public."Action" USING btree (name, description, quest_id);

INSERT INTO "public"."Action" ("id", "name", "region_name", "leyline_id", "quest_id", "description", "cost", "npc_id") VALUES
('2a93495e-d3ba-4cb6-8af4-8416fb0ae051', 'Explore', 'Dusane', NULL, 'be885fb0-4f60-4f9d-9a75-ceb4160a7f6a', NULL, 13, NULL);

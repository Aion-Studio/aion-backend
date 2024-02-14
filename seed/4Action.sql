-- -------------------------------------------------------------
-- TablePlus 5.8.6(534)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-02-12 10:30:17.7110 AM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

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
    CONSTRAINT "Action_quest_id_fkey" FOREIGN KEY ("quest_id") REFERENCES "public"."Quest"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "Action_leyline_id_fkey" FOREIGN KEY ("leyline_id") REFERENCES "public"."Leyline"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "Action_region_name_fkey" FOREIGN KEY ("region_name") REFERENCES "public"."Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "Action_npc_id_fkey" FOREIGN KEY ("npc_id") REFERENCES "public"."Npc"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Action" ("id", "name", "region_name", "leyline_id", "quest_id", "description", "cost", "npc_id") VALUES
('42eeec71-3d4b-4f64-a46d-c33f5087b712', 'FightNpc', 'Dusane', NULL, NULL, 'Fight the monster in forrest', 0, 'ef5666e9-95ee-4097-a643-afef984efee5'),
('ebc7b02e-91c8-4d8c-a931-2c3ca3f4d74b', 'Explore', 'Dusane', NULL, 'be885fb0-4f60-4f9d-9a75-ceb4160a7f6a', NULL, 6, NULL);

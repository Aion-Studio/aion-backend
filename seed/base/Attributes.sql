-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-05-16 8:26:51.6570 AM
-- -------------------------------------------------------------


DROP TABLE IF EXISTS "public"."Attributes";
-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Attributes" (
    "id" text NOT NULL,
    "strength" int4 NOT NULL,
    "agility" int4 NOT NULL,
    "intelligence" int4 NOT NULL,
    "exploration" int4 NOT NULL,
    "crafting" int4 NOT NULL,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Attributes" ("id", "strength", "agility", "intelligence", "exploration", "crafting") VALUES
('20d70fd0-c57c-4784-b401-12e788444eb7', 7, 1, 18, 18, 6),
('5756ea3d-d3b9-406f-9418-f571c0e6e7b5', 9, 18, 8, 9, 11),
('68bbfe10-d82c-43b7-ada3-4da70089f180', 18, 11, 18, 19, 18),
('9b7e233b-66b6-4e2f-9602-547963b2aa94', 9, 20, 17, 18, 19),
('debe7b0e-15c3-4f56-891e-aa0d96c00c9a', 5, 6, 4, 2, 5);

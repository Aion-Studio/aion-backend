-- -------------------------------------------------------------
-- TablePlus 5.5.2(512)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2023-10-25 6:07:07.5020 PM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."CommonEnum";
CREATE TYPE "public"."CommonEnum" AS ENUM ('IronOre', 'RoughLeather', 'Quartz');

-- Table Definition
CREATE TABLE "public"."Common" (
    "id" text NOT NULL,
    "type" "public"."CommonEnum" NOT NULL,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Common" ("id", "type") VALUES
('58f15a9d-05c6-4b2a-ad76-085352a22205', 'Quartz'),
('e2f99427-ff18-4a8f-b92c-7c05f8230b3a', 'IronOre'),
('fae71fe8-5450-47c3-b4e5-f8e8a819a5eb', 'RoughLeather');

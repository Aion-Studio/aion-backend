-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-05-16 8:27:46.8600 AM
-- -------------------------------------------------------------


DROP TABLE IF EXISTS "public"."Quest";
-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Quest" (
    "id" text NOT NULL,
    "title" text NOT NULL,
    "region_name" text NOT NULL,
    "questNumber" int4 NOT NULL,
    "cost" int4 NOT NULL DEFAULT 0,
    CONSTRAINT "Quest_region_name_fkey" FOREIGN KEY ("region_name") REFERENCES "public"."Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Quest" ("id", "title", "region_name", "questNumber", "cost") VALUES
('be885fb0-4f60-4f9d-9a75-ceb4160a7f6a', 'Obtain the medallion', 'Dusane', 1, 15);

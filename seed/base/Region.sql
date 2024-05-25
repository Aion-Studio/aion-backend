-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-05-16 8:28:12.7350 AM
-- -------------------------------------------------------------


DROP TABLE IF EXISTS "public"."Region";
-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Region" (
    "name" text NOT NULL,
    "adjacent_regions" _text,
    PRIMARY KEY ("name")
);

INSERT INTO "public"."Region" ("name", "adjacent_regions") VALUES
('Buzna', '{Lindon}'),
('Dusane', '{Veladria}'),
('Emerlad', '{Veladria,Lindon,Yezer,Forest}'),
('Forest', '{Yezer,Emerlad}'),
('Lindon', '{Veladria,Lindon,Yezer,Buzna}'),
('Veladria', '{Dusane,Lindon,Emerlad}'),
('Yezer', '{Lindon,Emerlad,Forest}');

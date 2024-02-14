-- -------------------------------------------------------------
-- TablePlus 5.5.2(512)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2023-10-25 6:08:04.2380 PM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

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

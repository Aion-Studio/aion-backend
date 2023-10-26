-- -------------------------------------------------------------
-- TablePlus 5.5.2(512)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2023-10-25 6:07:45.9960 PM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."RareEnum";
CREATE TYPE "public"."RareEnum" AS ENUM ('SilverOre', 'FineLeather', 'Sapphire');

-- Table Definition
CREATE TABLE "public"."Rare" (
    "id" text NOT NULL,
    "type" "public"."RareEnum" NOT NULL,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Rare" ("id", "type") VALUES
('03b4d83d-c670-42f0-bb21-9f9a0cfb535c', 'SilverOre'),
('9ad7b008-a313-4ce1-a2e8-bff10b8be4cd', 'FineLeather'),
('ba9c19ad-e546-4944-b762-836cf947fb4f', 'Sapphire');

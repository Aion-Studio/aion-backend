-- -------------------------------------------------------------
-- TablePlus 5.5.2(512)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2023-10-25 6:08:38.1050 PM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."MaterialEnum";
CREATE TYPE "public"."MaterialEnum" AS ENUM ('Common', 'Rare', 'Epic');

-- Table Definition
CREATE TABLE "public"."MaterialType" (
    "id" text NOT NULL,
    "type" "public"."MaterialEnum" NOT NULL,
    "commonId" text,
    "rareId" text,
    "epicId" text,
    CONSTRAINT "MaterialType_epicId_fkey" FOREIGN KEY ("epicId") REFERENCES "public"."Epic"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "MaterialType_commonId_fkey" FOREIGN KEY ("commonId") REFERENCES "public"."Common"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "MaterialType_rareId_fkey" FOREIGN KEY ("rareId") REFERENCES "public"."Rare"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."MaterialType" ("id", "type", "commonId", "rareId", "epicId") VALUES
('05e492a1-e821-4acb-92db-6f5d0adcdf55', 'Rare', NULL, '9ad7b008-a313-4ce1-a2e8-bff10b8be4cd', NULL),
('088661f9-0e9a-46a7-8fcc-0a3d85a6607f', 'Common', '58f15a9d-05c6-4b2a-ad76-085352a22205', NULL, NULL),
('411b2070-a2bd-4cbd-acf4-7e11b213aa67', 'Rare', NULL, '03b4d83d-c670-42f0-bb21-9f9a0cfb535c', NULL),
('484e13fd-45fb-43c1-a7df-e7c696bb1df1', 'Common', 'e2f99427-ff18-4a8f-b92c-7c05f8230b3a', NULL, NULL),
('4d276677-2247-4a59-86e2-0cdabade306e', 'Epic', NULL, NULL, '80674dc0-f56e-48d7-ae3b-bcb664487dbd'),
('6431f230-cc28-4295-a7b1-cf95ba87a395', 'Rare', NULL, 'ba9c19ad-e546-4944-b762-836cf947fb4f', NULL),
('8bfb5d5e-8986-4858-9257-29fa2c248106', 'Epic', NULL, NULL, '467232c1-6bde-42ba-8f08-dcb254bc6163'),
('a1162bf2-2221-4139-9b9f-0a58ed48c2d7', 'Common', 'fae71fe8-5450-47c3-b4e5-f8e8a819a5eb', NULL, NULL),
('b69c419b-95ec-4206-afa3-054e5706175a', 'Epic', NULL, NULL, 'f276e492-b803-46f7-9461-611753a28d52');

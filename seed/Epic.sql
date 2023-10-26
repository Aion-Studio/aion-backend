-- -------------------------------------------------------------
-- TablePlus 5.5.2(512)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2023-10-25 6:08:18.9990 PM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."EpicEnum";
CREATE TYPE "public"."EpicEnum" AS ENUM ('MythrilOre', 'Dragonhide', 'Ruby');

-- Table Definition
CREATE TABLE "public"."Epic" (
    "id" text NOT NULL,
    "type" "public"."EpicEnum" NOT NULL,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Epic" ("id", "type") VALUES
('467232c1-6bde-42ba-8f08-dcb254bc6163', 'Dragonhide'),
('80674dc0-f56e-48d7-ae3b-bcb664487dbd', 'MythrilOre'),
('f276e492-b803-46f7-9461-611753a28d52', 'Ruby');

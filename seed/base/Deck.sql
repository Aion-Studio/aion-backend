-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-06-27 8:53:05.0820 AM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Deck" (
    "id" text NOT NULL,
    "name" text NOT NULL DEFAULT 'my deck'::text,
    "heroId" text,
    "active" bool NOT NULL DEFAULT false,
    "npcId" text,
    CONSTRAINT "Deck_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "public"."Hero"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Deck" ("id", "name", "heroId", "active", "npcId") VALUES
('2e872d1f-83bf-41c8-9e11-c6ec7785a110', 'my deck', '90e64955-1208-4e8d-ab5e-04b3dbe86b6b', 'f', NULL),
('a741ac20-7cb0-4cb1-a423-22a99113ebc1', 'my deck', '2a038e2f-4de8-4907-b48a-c5ce89f876e3', 'f', NULL),
('b0ec518a-4232-4ad2-9155-4bba2905c325', 'my deck', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', 't', NULL);

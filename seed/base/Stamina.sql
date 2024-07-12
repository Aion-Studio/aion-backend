-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-06-27 8:54:14.6410 AM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Stamina" (
    "id" text NOT NULL,
    "capacity" int4 NOT NULL,
    "amount" int4 NOT NULL,
    "lastRegenTime" timestamp(3),
    "heroId" text,
    CONSTRAINT "Stamina_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "public"."Hero"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

-- Indices
CREATE UNIQUE INDEX "Stamina_heroId_key" ON public."Stamina" USING btree ("heroId");

INSERT INTO "public"."Stamina" ("id", "capacity", "amount", "lastRegenTime", "heroId") VALUES
('1f150689-c8d2-4e21-9395-667db99bf22b', 100, 100, NULL, '90e64955-1208-4e8d-ab5e-04b3dbe86b6b'),
('c0c116d3-c1cc-4eff-99c5-631d49515c8f', 100, 100, NULL, '2a038e2f-4de8-4907-b48a-c5ce89f876e3'),
('d198ac29-cdaa-4731-85e2-c33a9c3d685f', 0, 0, NULL, 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e');

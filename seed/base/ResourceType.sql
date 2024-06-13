-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-05-16 8:26:13.3640 AM
-- -------------------------------------------------------------


DROP TABLE IF EXISTS "public"."ResourceType";
-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."ResourceEnum";
CREATE TYPE "public"."ResourceEnum" AS ENUM ('Aion', 'Valor', 'NexusOrb', 'StormShard');

-- Table Definition
CREATE TABLE "public"."ResourceType" (
    "id" text NOT NULL,
    "type" "public"."ResourceEnum" NOT NULL,
    PRIMARY KEY ("id")
);

-- Indices
CREATE UNIQUE INDEX "ResourceType_type_key" ON public."ResourceType" USING btree (type);

INSERT INTO "public"."ResourceType" ("id", "type") VALUES
('105d5452-bfe0-44c6-893d-18cad6192eee', 'NexusOrb'),
('4b92d9a3-8d30-4895-babb-066ef85a3b4c', 'StormShard'),
('64bb06f5-04e4-42d9-97f0-28f31f89b370', 'Valor'),
('e1059285-3b37-4224-8bd9-5653cb8c3586', 'Aion');

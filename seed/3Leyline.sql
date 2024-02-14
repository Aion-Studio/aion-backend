-- -------------------------------------------------------------
-- TablePlus 5.5.2(512)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2023-10-25 6:07:26.3240 PM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Leyline" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "xp_reward" int4 NOT NULL,
    "RegionName" text NOT NULL,
    "aion_rate" float8 NOT NULL,
    "discovery_required" int4 NOT NULL,
    "stamina_rate" float8 NOT NULL,
    CONSTRAINT "Leyline_RegionName_fkey" FOREIGN KEY ("RegionName") REFERENCES "public"."Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Leyline" ("id", "name", "xp_reward", "RegionName", "aion_rate", "discovery_required", "stamina_rate") VALUES
('06a706a3-4f74-407a-99c1-a57a634064b8', 'Dusarock', 10, 'Dusane', 8, 0, 4),
('26c0affd-99d0-4a1d-9cb5-2900e1b41aeb', 'Dusaglow', 14, 'Dusane', 4, 22, 8),
('4f44727f-cd0e-4b83-949e-3d73ac07173a', 'Dusafire', 11, 'Dusane', 6, 45, 5),
('6204d298-34df-487e-9267-e6aed27cf2b8', 'Dusawater', 9, 'Dusane', 2, 40, 8),
('62fbfa8c-e1ce-48a9-9d21-2e0e834f000c', 'Dusadream', 12, 'Dusane', 7, 15, 3),
('65980570-2d94-4a5d-a4fb-250791bc4381', 'Dusacloud', 11, 'Dusane', 7, 72, 4),
('7699b133-1762-4eb6-b4a8-ca6f97daabd0', 'Dusaearth', 8, 'Dusane', 10, 41, 3),
('cea699bf-8650-461f-9311-1f8b112a3027', 'Dusawind', 17, 'Dusane', 4, 85, 4),
('d17c2a46-28db-4b68-ab6b-635f46b5bbda', 'Dusalight', 3, 'Dusane', 5, 5, 6),
('d5e9a122-e053-44f7-9561-e07006d354f8', 'Dusaspark', 6, 'Dusane', 2, 61, 9);

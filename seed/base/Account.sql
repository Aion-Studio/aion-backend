-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-09-25 12:30:22.0510 PM
-- -------------------------------------------------------------


DROP TABLE IF EXISTS "public"."Account";
-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Account" (
    "id" text NOT NULL,
    "supabase_user_id" text NOT NULL,
    "created_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

-- Indices
CREATE UNIQUE INDEX "Account_supabase_user_id_key" ON public."Account" USING btree (supabase_user_id);

INSERT INTO "public"."Account" ("id", "supabase_user_id", "created_at", "updated_at") VALUES
('b6ee7843-bbd9-4937-8272-5c633e03b880', 'some_user', '2024-09-25 10:28:29.268', '2024-09-25 10:28:29.268'),
('c081e311-abe2-49f6-a48d-26dc4dbd1f53', 'secondId', '2024-09-25 10:28:17.81', '2024-09-25 10:28:17.81'),
('f42dc222-0dd2-43c4-b581-c36ded16bac5', 'marko911', '2024-09-25 10:27:34.298', '2024-09-25 10:27:34.298');

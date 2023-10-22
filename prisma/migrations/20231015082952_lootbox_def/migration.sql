/*
  Warnings:

  - Made the column `lootBox` on table `ActionCompleted` required. This step will fail if there are existing NULL values in that column.

*/

-- Update all NULL values to the default value
UPDATE "ActionCompleted" SET "lootBox" = '{}' WHERE "lootBox" IS NULL;

-- AlterTable
ALTER TABLE "ActionCompleted" ALTER COLUMN "lootBox" SET NOT NULL,
ALTER COLUMN "lootBox" SET DEFAULT '{}';

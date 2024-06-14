/*
  Warnings:

  - Made the column `inDeck` on table `HeroCard` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "HeroCard" ALTER COLUMN "instanceId" SET DATA TYPE TEXT,
ALTER COLUMN "inDeck" SET NOT NULL;

/*
  Warnings:

  - You are about to drop the column `regenRate` on the `Stamina` table. All the data in the column will be lost.

*/
-- DropForeignKey
ALTER TABLE "Deck" DROP CONSTRAINT "Deck_npcId_fkey";

-- AlterTable
ALTER TABLE "Stamina" DROP COLUMN "regenRate";

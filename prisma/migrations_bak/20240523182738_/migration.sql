/*
  Warnings:

  - You are about to drop the column `cardId` on the `DeckCard` table. All the data in the column will be lost.

*/
-- DropForeignKey
ALTER TABLE "DeckCard" DROP CONSTRAINT "DeckCard_cardId_fkey";

-- AlterTable
ALTER TABLE "DeckCard" DROP COLUMN "cardId";

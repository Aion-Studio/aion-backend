/*
  Warnings:

  - You are about to drop the column `quantity` on the `DeckCard` table. All the data in the column will be lost.

*/
-- DropIndex
DROP INDEX "DeckCard_deckId_cardId_key";

-- AlterTable
ALTER TABLE "DeckCard" DROP COLUMN "quantity";

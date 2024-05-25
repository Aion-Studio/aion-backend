/*
  Warnings:

  - A unique constraint covering the columns `[deckCardId]` on the table `HeroCard` will be added. If there are existing duplicate values, this will fail.

*/
-- DropForeignKey
ALTER TABLE "DeckCard" DROP CONSTRAINT "DeckCard_cardId_fkey";

-- AlterTable
ALTER TABLE "DeckCard" ALTER COLUMN "cardId" DROP NOT NULL;

-- AlterTable
ALTER TABLE "HeroCard" ADD COLUMN     "deckCardId" TEXT;

-- CreateIndex
CREATE UNIQUE INDEX "HeroCard_deckCardId_key" ON "HeroCard"("deckCardId");

-- AddForeignKey
ALTER TABLE "HeroCard" ADD CONSTRAINT "HeroCard_deckCardId_fkey" FOREIGN KEY ("deckCardId") REFERENCES "DeckCard"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DeckCard" ADD CONSTRAINT "DeckCard_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE SET NULL ON UPDATE CASCADE;

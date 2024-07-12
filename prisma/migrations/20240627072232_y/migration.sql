/*
  Warnings:

  - A unique constraint covering the columns `[heroCardId]` on the table `DeckCard` will be added. If there are existing duplicate values, this will fail.
  - Made the column `heroCardId` on table `DeckCard` required. This step will fail if there are existing NULL values in that column.

*/
-- DropForeignKey
ALTER TABLE "DeckCard" DROP CONSTRAINT "DeckCard_heroCardId_fkey";

-- AlterTable
ALTER TABLE "DeckCard" ALTER COLUMN "heroCardId" SET NOT NULL;

-- AlterTable
ALTER TABLE "HeroCard" ADD COLUMN     "inDeck" BOOLEAN NOT NULL DEFAULT false;

-- CreateIndex
CREATE UNIQUE INDEX "DeckCard_heroCardId_key" ON "DeckCard"("heroCardId");

-- AddForeignKey
ALTER TABLE "DeckCard" ADD CONSTRAINT "DeckCard_deckId_fkey" FOREIGN KEY ("deckId") REFERENCES "Deck"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DeckCard" ADD CONSTRAINT "DeckCard_heroCardId_fkey" FOREIGN KEY ("heroCardId") REFERENCES "HeroCard"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

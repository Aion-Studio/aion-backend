/*
  Warnings:

  - You are about to drop the column `deckId` on the `Hero` table. All the data in the column will be lost.
  - You are about to drop the column `inDeck` on the `HeroCard` table. All the data in the column will be lost.
  - You are about to drop the column `inDeck` on the `NpcCard` table. All the data in the column will be lost.

*/
-- DropForeignKey
ALTER TABLE "Hero" DROP CONSTRAINT "Hero_deckId_fkey";

-- DropIndex
DROP INDEX "Hero_deckId_key";

-- AlterTable
ALTER TABLE "Deck" ADD COLUMN     "heroId" TEXT;

-- AlterTable
ALTER TABLE "Hero" DROP COLUMN "deckId";

-- AlterTable
ALTER TABLE "HeroCard" DROP COLUMN "inDeck";

-- AlterTable
ALTER TABLE "NpcCard" DROP COLUMN "inDeck";

-- CreateTable
CREATE TABLE "DeckCard" (
    "id" TEXT NOT NULL,
    "deckId" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,

    CONSTRAINT "DeckCard_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "DeckCard" ADD CONSTRAINT "DeckCard_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Deck" ADD CONSTRAINT "Deck_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE SET NULL ON UPDATE CASCADE;

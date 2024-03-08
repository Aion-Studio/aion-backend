/*
  Warnings:

  - You are about to drop the column `heroId` on the `Card` table. All the data in the column will be lost.
  - The primary key for the `CardInDeck` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `cardId` on the `CardInDeck` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[heroCardId]` on the table `CardInDeck` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `heroCardId` to the `CardInDeck` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "Card" DROP CONSTRAINT "Card_heroId_fkey";

-- DropForeignKey
ALTER TABLE "CardInDeck" DROP CONSTRAINT "CardInDeck_cardId_fkey";

-- DropIndex
DROP INDEX "CardInDeck_cardId_key";

-- AlterTable
ALTER TABLE "Card" DROP COLUMN "heroId";

-- AlterTable
ALTER TABLE "CardInDeck" DROP CONSTRAINT "CardInDeck_pkey",
DROP COLUMN "cardId",
ADD COLUMN     "heroCardId" TEXT NOT NULL,
ADD CONSTRAINT "CardInDeck_pkey" PRIMARY KEY ("deckId", "heroCardId");

-- CreateTable
CREATE TABLE "HeroCard" (
    "id" TEXT NOT NULL,
    "heroId" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,

    CONSTRAINT "HeroCard_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "HeroCard_heroId_cardId_key" ON "HeroCard"("heroId", "cardId");

-- CreateIndex
CREATE UNIQUE INDEX "CardInDeck_heroCardId_key" ON "CardInDeck"("heroCardId");

-- AddForeignKey
ALTER TABLE "HeroCard" ADD CONSTRAINT "HeroCard_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroCard" ADD CONSTRAINT "HeroCard_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CardInDeck" ADD CONSTRAINT "CardInDeck_heroCardId_fkey" FOREIGN KEY ("heroCardId") REFERENCES "HeroCard"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

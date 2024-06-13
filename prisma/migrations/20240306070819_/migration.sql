/*
  Warnings:

  - A unique constraint covering the columns `[cardId]` on the table `CardInDeck` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX "CardInDeck_cardId_key" ON "CardInDeck"("cardId");

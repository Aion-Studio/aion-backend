/*
  Warnings:

  - A unique constraint covering the columns `[cardId,effectId]` on the table `CardEffect` will be added. If there are existing duplicate values, this will fail.

*/
-- DropIndex
DROP INDEX "CardEffect_effectId_key";

-- CreateIndex
CREATE UNIQUE INDEX "CardEffect_cardId_effectId_key" ON "CardEffect"("cardId", "effectId");

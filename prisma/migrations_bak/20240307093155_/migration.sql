/*
  Warnings:

  - A unique constraint covering the columns `[effectId]` on the table `CardEffect` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX "CardEffect_effectId_key" ON "CardEffect"("effectId");

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_effectId_fkey" FOREIGN KEY ("effectId") REFERENCES "Effect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

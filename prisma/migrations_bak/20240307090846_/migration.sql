/*
  Warnings:

  - A unique constraint covering the columns `[effectId]` on the table `CardEffect` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateTable
CREATE TABLE "EffectsInCardEffect" (
    "id" TEXT NOT NULL,
    "effectId" TEXT NOT NULL,
    "cardEffectId" TEXT NOT NULL,

    CONSTRAINT "EffectsInCardEffect_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "EffectsInCardEffect_effectId_key" ON "EffectsInCardEffect"("effectId");

-- CreateIndex
CREATE UNIQUE INDEX "CardEffect_effectId_key" ON "CardEffect"("effectId");

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_effectId_fkey" FOREIGN KEY ("effectId") REFERENCES "EffectsInCardEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "EffectsInCardEffect" ADD CONSTRAINT "EffectsInCardEffect_effectId_fkey" FOREIGN KEY ("effectId") REFERENCES "Effect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

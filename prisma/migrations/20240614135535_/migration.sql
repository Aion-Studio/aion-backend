/*
  Warnings:

  - You are about to drop the column `effects` on the `Card` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Card" DROP COLUMN "effects";

-- CreateTable
CREATE TABLE "CardEffect" (
    "id" SERIAL NOT NULL,
    "type" "EffectType" NOT NULL,
    "value" INTEGER NOT NULL,
    "target" TEXT NOT NULL,
    "duration" INTEGER,
    "cardId" TEXT NOT NULL,

    CONSTRAINT "CardEffect_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

/*
  Warnings:

  - You are about to drop the column `duration` on the `Effect` table. All the data in the column will be lost.
  - You are about to drop the column `effect` on the `Effect` table. All the data in the column will be lost.
  - You are about to drop the column `value` on the `Effect` table. All the data in the column will be lost.
  - You are about to drop the `_CardEffects` table. If the table is not empty, all the data it contains will be lost.
  - Added the required column `effectType` to the `Effect` table without a default value. This is not possible if the table is not empty.

*/
-- CreateEnum
CREATE TYPE "DamageType" AS ENUM ('Physical', 'Spell', 'Chaos');

-- DropForeignKey
ALTER TABLE "_CardEffects" DROP CONSTRAINT "_CardEffects_A_fkey";

-- DropForeignKey
ALTER TABLE "_CardEffects" DROP CONSTRAINT "_CardEffects_B_fkey";

-- AlterTable
ALTER TABLE "Card" ADD COLUMN     "damageType" "DamageType",
ADD COLUMN     "damageValue" INTEGER;

-- AlterTable
ALTER TABLE "Effect" DROP COLUMN "duration",
DROP COLUMN "effect",
DROP COLUMN "value",
ADD COLUMN     "effectType" "EffectType" NOT NULL;

-- DropTable
DROP TABLE "_CardEffects";

-- CreateTable
CREATE TABLE "CardEffect" (
    "id" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,
    "effectId" TEXT NOT NULL,
    "value" INTEGER,
    "duration" INTEGER,

    CONSTRAINT "CardEffect_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "CardEffect_cardId_effectId_key" ON "CardEffect"("cardId", "effectId");

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_effectId_fkey" FOREIGN KEY ("effectId") REFERENCES "Effect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

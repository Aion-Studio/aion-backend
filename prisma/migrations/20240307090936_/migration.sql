/*
  Warnings:

  - You are about to drop the `EffectsInCardEffect` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "CardEffect" DROP CONSTRAINT "CardEffect_effectId_fkey";

-- DropForeignKey
ALTER TABLE "EffectsInCardEffect" DROP CONSTRAINT "EffectsInCardEffect_effectId_fkey";

-- DropIndex
DROP INDEX "CardEffect_effectId_key";

-- DropTable
DROP TABLE "EffectsInCardEffect";

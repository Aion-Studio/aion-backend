/*
  Warnings:

  - You are about to drop the `CardEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `EffectType` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "CardEffect" DROP CONSTRAINT "CardEffect_cardId_fkey";

-- AlterTable
ALTER TABLE "Card" ADD COLUMN     "duration" INTEGER NOT NULL DEFAULT 0,
ADD COLUMN     "effectType" "EffectEnum" NOT NULL DEFAULT 'PhysicalDamage';

-- DropTable
DROP TABLE "CardEffect";

-- DropTable
DROP TABLE "EffectType";

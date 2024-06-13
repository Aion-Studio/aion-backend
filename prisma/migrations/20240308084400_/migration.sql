/*
  Warnings:

  - You are about to drop the column `cardEffectId` on the `ArmorEffect` table. All the data in the column will be lost.
  - You are about to drop the column `cardEffectId` on the `DamageEffect` table. All the data in the column will be lost.
  - You are about to drop the column `cardEffectId` on the `HealEffect` table. All the data in the column will be lost.
  - You are about to drop the column `cardEffectId` on the `InitiativeEffect` table. All the data in the column will be lost.
  - You are about to drop the column `cardEffectId` on the `PoisonEffect` table. All the data in the column will be lost.
  - You are about to drop the column `cardEffectId` on the `ResilienceEffect` table. All the data in the column will be lost.
  - You are about to drop the column `cardEffectId` on the `StunEffect` table. All the data in the column will be lost.

*/
-- DropForeignKey
ALTER TABLE "ArmorEffect" DROP CONSTRAINT "ArmorEffect_cardEffectId_fkey";

-- DropForeignKey
ALTER TABLE "DamageEffect" DROP CONSTRAINT "DamageEffect_cardEffectId_fkey";

-- DropForeignKey
ALTER TABLE "HealEffect" DROP CONSTRAINT "HealEffect_cardEffectId_fkey";

-- DropForeignKey
ALTER TABLE "InitiativeEffect" DROP CONSTRAINT "InitiativeEffect_cardEffectId_fkey";

-- DropForeignKey
ALTER TABLE "PoisonEffect" DROP CONSTRAINT "PoisonEffect_cardEffectId_fkey";

-- DropForeignKey
ALTER TABLE "ResilienceEffect" DROP CONSTRAINT "ResilienceEffect_cardEffectId_fkey";

-- DropForeignKey
ALTER TABLE "StunEffect" DROP CONSTRAINT "StunEffect_cardEffectId_fkey";

-- DropIndex
DROP INDEX "ArmorEffect_cardEffectId_key";

-- DropIndex
DROP INDEX "DamageEffect_cardEffectId_key";

-- DropIndex
DROP INDEX "HealEffect_cardEffectId_key";

-- DropIndex
DROP INDEX "InitiativeEffect_cardEffectId_key";

-- DropIndex
DROP INDEX "PoisonEffect_cardEffectId_key";

-- DropIndex
DROP INDEX "ResilienceEffect_cardEffectId_key";

-- DropIndex
DROP INDEX "StunEffect_cardEffectId_key";

-- AlterTable
ALTER TABLE "ArmorEffect" DROP COLUMN "cardEffectId";

-- AlterTable
ALTER TABLE "DamageEffect" DROP COLUMN "cardEffectId";

-- AlterTable
ALTER TABLE "HealEffect" DROP COLUMN "cardEffectId";

-- AlterTable
ALTER TABLE "InitiativeEffect" DROP COLUMN "cardEffectId";

-- AlterTable
ALTER TABLE "PoisonEffect" DROP COLUMN "cardEffectId";

-- AlterTable
ALTER TABLE "ResilienceEffect" DROP COLUMN "cardEffectId";

-- AlterTable
ALTER TABLE "SpellEffect" ADD COLUMN     "armorEffectId" TEXT,
ADD COLUMN     "damageEffectId" TEXT,
ADD COLUMN     "healEffectId" TEXT,
ADD COLUMN     "initiativeEffectId" TEXT,
ADD COLUMN     "poisonEffectId" TEXT,
ADD COLUMN     "resilienceEffectId" TEXT,
ADD COLUMN     "stunEffectId" TEXT;

-- AlterTable
ALTER TABLE "StunEffect" DROP COLUMN "cardEffectId";

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_damageEffectId_fkey" FOREIGN KEY ("damageEffectId") REFERENCES "DamageEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_healEffectId_fkey" FOREIGN KEY ("healEffectId") REFERENCES "HealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_armorEffectId_fkey" FOREIGN KEY ("armorEffectId") REFERENCES "ArmorEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_resilienceEffectId_fkey" FOREIGN KEY ("resilienceEffectId") REFERENCES "ResilienceEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_poisonEffectId_fkey" FOREIGN KEY ("poisonEffectId") REFERENCES "PoisonEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_initiativeEffectId_fkey" FOREIGN KEY ("initiativeEffectId") REFERENCES "InitiativeEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_stunEffectId_fkey" FOREIGN KEY ("stunEffectId") REFERENCES "StunEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

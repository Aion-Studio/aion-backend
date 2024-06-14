/*
  Warnings:

  - You are about to drop the column `armorEffectId` on the `SpellEffect` table. All the data in the column will be lost.
  - You are about to drop the column `damageEffectId` on the `SpellEffect` table. All the data in the column will be lost.
  - You are about to drop the column `healEffectId` on the `SpellEffect` table. All the data in the column will be lost.
  - You are about to drop the column `initiativeEffectId` on the `SpellEffect` table. All the data in the column will be lost.
  - You are about to drop the column `poisonEffectId` on the `SpellEffect` table. All the data in the column will be lost.
  - You are about to drop the column `resilienceEffectId` on the `SpellEffect` table. All the data in the column will be lost.
  - You are about to drop the column `stunEffectId` on the `SpellEffect` table. All the data in the column will be lost.

*/
-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_armorEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_damageEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_healEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_initiativeEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_poisonEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_resilienceEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_stunEffectId_fkey";

-- AlterTable
ALTER TABLE "SpellEffect" DROP COLUMN "armorEffectId",
DROP COLUMN "damageEffectId",
DROP COLUMN "healEffectId",
DROP COLUMN "initiativeEffectId",
DROP COLUMN "poisonEffectId",
DROP COLUMN "resilienceEffectId",
DROP COLUMN "stunEffectId";

-- CreateTable
CREATE TABLE "SpellEffectEffect" (
    "id" TEXT NOT NULL,
    "spellEffectId" TEXT NOT NULL,
    "damageEffectId" TEXT,
    "healEffectId" TEXT,
    "armorEffectId" TEXT,
    "resilienceEffectId" TEXT,
    "poisonEffectId" TEXT,
    "initiativeEffectId" TEXT,
    "stunEffectId" TEXT,

    CONSTRAINT "SpellEffectEffect_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_damageEffectId_fkey" FOREIGN KEY ("damageEffectId") REFERENCES "DamageEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_healEffectId_fkey" FOREIGN KEY ("healEffectId") REFERENCES "HealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_armorEffectId_fkey" FOREIGN KEY ("armorEffectId") REFERENCES "ArmorEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_resilienceEffectId_fkey" FOREIGN KEY ("resilienceEffectId") REFERENCES "ResilienceEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_poisonEffectId_fkey" FOREIGN KEY ("poisonEffectId") REFERENCES "PoisonEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_initiativeEffectId_fkey" FOREIGN KEY ("initiativeEffectId") REFERENCES "InitiativeEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_stunEffectId_fkey" FOREIGN KEY ("stunEffectId") REFERENCES "StunEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

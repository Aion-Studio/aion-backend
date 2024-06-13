/*
  Warnings:

  - You are about to drop the `CardEffect` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "ArmorEffect" DROP CONSTRAINT "ArmorEffect_cardEffectId_fkey";

-- DropForeignKey
ALTER TABLE "CardEffect" DROP CONSTRAINT "CardEffect_cardId_fkey";

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

-- DropTable
DROP TABLE "CardEffect";

-- CreateTable
CREATE TABLE "SpellEffect" (
    "id" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,
    "duration" INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT "SpellEffect_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DamageEffect" ADD CONSTRAINT "DamageEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HealEffect" ADD CONSTRAINT "HealEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ArmorEffect" ADD CONSTRAINT "ArmorEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ResilienceEffect" ADD CONSTRAINT "ResilienceEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PoisonEffect" ADD CONSTRAINT "PoisonEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "InitiativeEffect" ADD CONSTRAINT "InitiativeEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StunEffect" ADD CONSTRAINT "StunEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

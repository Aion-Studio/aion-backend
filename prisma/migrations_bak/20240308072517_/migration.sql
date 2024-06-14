-- DropForeignKey
ALTER TABLE "ArmorEffect" DROP CONSTRAINT "ArmorEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "DamageEffect" DROP CONSTRAINT "DamageEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "HealEffect" DROP CONSTRAINT "HealEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "InitiativeEffect" DROP CONSTRAINT "InitiativeEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "PoisonEffect" DROP CONSTRAINT "PoisonEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "ResilienceEffect" DROP CONSTRAINT "ResilienceEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "StunEffect" DROP CONSTRAINT "StunEffect_spellEffectId_fkey";

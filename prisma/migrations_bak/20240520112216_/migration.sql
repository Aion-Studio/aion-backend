-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_blockEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_chargeEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_cleanseEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_dyingWishDamageEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_dyingWishHealEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_etherealEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_lifestealEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_pickupEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_roarAuraEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_tauntEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_twinEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_armorEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_battleCryEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_cowardiceCurseEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_damageEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_dazeEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_healEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_initiativeEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_phantomTouchEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_poisonEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_resilienceEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_sprayOfKnivesEffectId_fkey";

-- AlterTable
ALTER TABLE "MinionEffect" ALTER COLUMN "blockEffectId" DROP NOT NULL,
ALTER COLUMN "chargeEffectId" DROP NOT NULL,
ALTER COLUMN "cleanseEffectId" DROP NOT NULL,
ALTER COLUMN "dyingWishDamageEffectId" DROP NOT NULL,
ALTER COLUMN "dyingWishHealEffectId" DROP NOT NULL,
ALTER COLUMN "etherealEffectId" DROP NOT NULL,
ALTER COLUMN "lifestealEffectId" DROP NOT NULL,
ALTER COLUMN "pickupEffectId" DROP NOT NULL,
ALTER COLUMN "roarAuraEffectId" DROP NOT NULL,
ALTER COLUMN "tauntEffectId" DROP NOT NULL,
ALTER COLUMN "twinEffectId" DROP NOT NULL;

-- AlterTable
ALTER TABLE "SpellEffect" ALTER COLUMN "armorEffectId" DROP NOT NULL,
ALTER COLUMN "battleCryEffectId" DROP NOT NULL,
ALTER COLUMN "cowardiceCurseEffectId" DROP NOT NULL,
ALTER COLUMN "damageEffectId" DROP NOT NULL,
ALTER COLUMN "dazeEffectId" DROP NOT NULL,
ALTER COLUMN "healEffectId" DROP NOT NULL,
ALTER COLUMN "initiativeEffectId" DROP NOT NULL,
ALTER COLUMN "phantomTouchEffectId" DROP NOT NULL,
ALTER COLUMN "poisonEffectId" DROP NOT NULL,
ALTER COLUMN "resilienceEffectId" DROP NOT NULL,
ALTER COLUMN "sprayOfKnivesEffectId" DROP NOT NULL;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_damageEffectId_fkey" FOREIGN KEY ("damageEffectId") REFERENCES "DamageEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_dazeEffectId_fkey" FOREIGN KEY ("dazeEffectId") REFERENCES "DazeEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_battleCryEffectId_fkey" FOREIGN KEY ("battleCryEffectId") REFERENCES "BattleCryEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_cowardiceCurseEffectId_fkey" FOREIGN KEY ("cowardiceCurseEffectId") REFERENCES "CowardiceCurseEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_phantomTouchEffectId_fkey" FOREIGN KEY ("phantomTouchEffectId") REFERENCES "PhantomTouchEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_sprayOfKnivesEffectId_fkey" FOREIGN KEY ("sprayOfKnivesEffectId") REFERENCES "SprayOfKnivesEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

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
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_etherealEffectId_fkey" FOREIGN KEY ("etherealEffectId") REFERENCES "EtherealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_twinEffectId_fkey" FOREIGN KEY ("twinEffectId") REFERENCES "TwinEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_cleanseEffectId_fkey" FOREIGN KEY ("cleanseEffectId") REFERENCES "CleanseEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_blockEffectId_fkey" FOREIGN KEY ("blockEffectId") REFERENCES "BlockEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_roarAuraEffectId_fkey" FOREIGN KEY ("roarAuraEffectId") REFERENCES "RoarAuraEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_dyingWishHealEffectId_fkey" FOREIGN KEY ("dyingWishHealEffectId") REFERENCES "DyingWishHealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_dyingWishDamageEffectId_fkey" FOREIGN KEY ("dyingWishDamageEffectId") REFERENCES "DyingWishDamageEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_tauntEffectId_fkey" FOREIGN KEY ("tauntEffectId") REFERENCES "TauntEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_chargeEffectId_fkey" FOREIGN KEY ("chargeEffectId") REFERENCES "ChargeEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_lifestealEffectId_fkey" FOREIGN KEY ("lifestealEffectId") REFERENCES "LifestealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_pickupEffectId_fkey" FOREIGN KEY ("pickupEffectId") REFERENCES "PickupEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

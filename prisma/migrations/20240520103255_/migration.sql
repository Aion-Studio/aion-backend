/*
  Warnings:

  - You are about to drop the column `spellEffectId` on the `ArmorEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `BattleCryEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `BlockEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `ChargeEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `CleanseEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `CowardiceCurseEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `DamageEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `DazeEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `DyingWishDamageEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `DyingWishHealEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `EtherealEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `HealEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `InitiativeEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `LifestealEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `PhantomTouchEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `PickupEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `PoisonEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `ResilienceEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `RoarAuraEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `SprayOfKnivesEffect` table. All the data in the column will be lost.
  - You are about to drop the column `spellEffectId` on the `StunEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `TauntEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `TwinEffect` table. All the data in the column will be lost.
  - Added the required column `blockEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `chargeEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `cleanseEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `dyingWishDamageEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `dyingWishHealEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `etherealEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `lifestealEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `pickupEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `roarAuraEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `tauntEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `twinEffectId` to the `MinionEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `armorEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `battleCryEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `cowardiceCurseEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `damageEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `dazeEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `healEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `initiativeEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `phantomTouchEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `poisonEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `resilienceEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `sprayOfKnivesEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `stunEffectId` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "ArmorEffect" DROP CONSTRAINT "ArmorEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "BattleCryEffect" DROP CONSTRAINT "BattleCryEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "BlockEffect" DROP CONSTRAINT "BlockEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "ChargeEffect" DROP CONSTRAINT "ChargeEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "CleanseEffect" DROP CONSTRAINT "CleanseEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "CowardiceCurseEffect" DROP CONSTRAINT "CowardiceCurseEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "DamageEffect" DROP CONSTRAINT "DamageEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "DazeEffect" DROP CONSTRAINT "DazeEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "DyingWishDamageEffect" DROP CONSTRAINT "DyingWishDamageEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "DyingWishHealEffect" DROP CONSTRAINT "DyingWishHealEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "EtherealEffect" DROP CONSTRAINT "EtherealEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "HealEffect" DROP CONSTRAINT "HealEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "InitiativeEffect" DROP CONSTRAINT "InitiativeEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "LifestealEffect" DROP CONSTRAINT "LifestealEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "PhantomTouchEffect" DROP CONSTRAINT "PhantomTouchEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "PickupEffect" DROP CONSTRAINT "PickupEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "PoisonEffect" DROP CONSTRAINT "PoisonEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "ResilienceEffect" DROP CONSTRAINT "ResilienceEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "RoarAuraEffect" DROP CONSTRAINT "RoarAuraEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SprayOfKnivesEffect" DROP CONSTRAINT "SprayOfKnivesEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "StunEffect" DROP CONSTRAINT "StunEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "TauntEffect" DROP CONSTRAINT "TauntEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "TwinEffect" DROP CONSTRAINT "TwinEffect_minionEffectId_fkey";

-- AlterTable
ALTER TABLE "ArmorEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "BattleCryEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "BlockEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "ChargeEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "CleanseEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "CowardiceCurseEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "DamageEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "DazeEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "DyingWishDamageEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "DyingWishHealEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "EtherealEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "HealEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "InitiativeEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "LifestealEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "MinionEffect" ADD COLUMN     "blockEffectId" TEXT NOT NULL,
ADD COLUMN     "chargeEffectId" TEXT NOT NULL,
ADD COLUMN     "cleanseEffectId" TEXT NOT NULL,
ADD COLUMN     "dyingWishDamageEffectId" TEXT NOT NULL,
ADD COLUMN     "dyingWishHealEffectId" TEXT NOT NULL,
ADD COLUMN     "etherealEffectId" TEXT NOT NULL,
ADD COLUMN     "lifestealEffectId" TEXT NOT NULL,
ADD COLUMN     "pickupEffectId" TEXT NOT NULL,
ADD COLUMN     "roarAuraEffectId" TEXT NOT NULL,
ADD COLUMN     "tauntEffectId" TEXT NOT NULL,
ADD COLUMN     "twinEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "PhantomTouchEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "PickupEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "PoisonEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "ResilienceEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "RoarAuraEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "SpellEffect" ADD COLUMN     "armorEffectId" TEXT NOT NULL,
ADD COLUMN     "battleCryEffectId" TEXT NOT NULL,
ADD COLUMN     "cowardiceCurseEffectId" TEXT NOT NULL,
ADD COLUMN     "damageEffectId" TEXT NOT NULL,
ADD COLUMN     "dazeEffectId" TEXT NOT NULL,
ADD COLUMN     "healEffectId" TEXT NOT NULL,
ADD COLUMN     "initiativeEffectId" TEXT NOT NULL,
ADD COLUMN     "phantomTouchEffectId" TEXT NOT NULL,
ADD COLUMN     "poisonEffectId" TEXT NOT NULL,
ADD COLUMN     "resilienceEffectId" TEXT NOT NULL,
ADD COLUMN     "sprayOfKnivesEffectId" TEXT NOT NULL,
ADD COLUMN     "stunEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "SprayOfKnivesEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "StunEffect" DROP COLUMN "spellEffectId";

-- AlterTable
ALTER TABLE "TauntEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "TwinEffect" DROP COLUMN "minionEffectId";

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_damageEffectId_fkey" FOREIGN KEY ("damageEffectId") REFERENCES "DamageEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_dazeEffectId_fkey" FOREIGN KEY ("dazeEffectId") REFERENCES "DazeEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_battleCryEffectId_fkey" FOREIGN KEY ("battleCryEffectId") REFERENCES "BattleCryEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_cowardiceCurseEffectId_fkey" FOREIGN KEY ("cowardiceCurseEffectId") REFERENCES "CowardiceCurseEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_phantomTouchEffectId_fkey" FOREIGN KEY ("phantomTouchEffectId") REFERENCES "PhantomTouchEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_sprayOfKnivesEffectId_fkey" FOREIGN KEY ("sprayOfKnivesEffectId") REFERENCES "SprayOfKnivesEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_healEffectId_fkey" FOREIGN KEY ("healEffectId") REFERENCES "HealEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_armorEffectId_fkey" FOREIGN KEY ("armorEffectId") REFERENCES "ArmorEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_resilienceEffectId_fkey" FOREIGN KEY ("resilienceEffectId") REFERENCES "ResilienceEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_poisonEffectId_fkey" FOREIGN KEY ("poisonEffectId") REFERENCES "PoisonEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_initiativeEffectId_fkey" FOREIGN KEY ("initiativeEffectId") REFERENCES "InitiativeEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_stunEffectId_fkey" FOREIGN KEY ("stunEffectId") REFERENCES "StunEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_etherealEffectId_fkey" FOREIGN KEY ("etherealEffectId") REFERENCES "EtherealEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_twinEffectId_fkey" FOREIGN KEY ("twinEffectId") REFERENCES "TwinEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_cleanseEffectId_fkey" FOREIGN KEY ("cleanseEffectId") REFERENCES "CleanseEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_blockEffectId_fkey" FOREIGN KEY ("blockEffectId") REFERENCES "BlockEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_roarAuraEffectId_fkey" FOREIGN KEY ("roarAuraEffectId") REFERENCES "RoarAuraEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_dyingWishHealEffectId_fkey" FOREIGN KEY ("dyingWishHealEffectId") REFERENCES "DyingWishHealEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_dyingWishDamageEffectId_fkey" FOREIGN KEY ("dyingWishDamageEffectId") REFERENCES "DyingWishDamageEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_tauntEffectId_fkey" FOREIGN KEY ("tauntEffectId") REFERENCES "TauntEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_chargeEffectId_fkey" FOREIGN KEY ("chargeEffectId") REFERENCES "ChargeEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_lifestealEffectId_fkey" FOREIGN KEY ("lifestealEffectId") REFERENCES "LifestealEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_pickupEffectId_fkey" FOREIGN KEY ("pickupEffectId") REFERENCES "PickupEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

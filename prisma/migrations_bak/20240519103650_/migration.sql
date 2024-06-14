/*
  Warnings:

  - You are about to drop the column `cardId` on the `MinionEffect` table. All the data in the column will be lost.
  - You are about to drop the column `cardId` on the `SpellEffect` table. All the data in the column will be lost.
  - You are about to drop the `MinionEffectEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `SpellEffectEffect` table. If the table is not empty, all the data it contains will be lost.
  - Added the required column `spellEffectId` to the `ArmorEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `BattleCryEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `BlockEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `ChargeEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `CleanseEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `CowardiceCurseEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `DamageEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `DazeEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `DyingWishDamageEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `DyingWishHealEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `EtherealEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `HealEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `InitiativeEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `LifestealEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `PhantomTouchEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `PickupEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `PoisonEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `ResilienceEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `RoarAuraEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `SprayOfKnivesEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `spellEffectId` to the `StunEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `TauntEffect` table without a default value. This is not possible if the table is not empty.
  - Added the required column `minionEffectId` to the `TwinEffect` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_blockEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_chargeEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_cleanseEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_dyingWishDamageEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_dyingWishHealEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_etherealEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_lifestealEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_pickupEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_roarAuraEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_tauntEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_twinEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_armorEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_battleCryEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_cowardiceCurseEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_damageEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_dazeEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_healEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_initiativeEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_phantomTouchEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_poisonEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_resilienceEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_sprayOfKnivesEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_stunEffectId_fkey";

-- AlterTable
ALTER TABLE "ArmorEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "BattleCryEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "BlockEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "ChargeEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "CleanseEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "CowardiceCurseEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "DamageEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "DazeEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "DyingWishDamageEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "DyingWishHealEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "EtherealEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "HealEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "InitiativeEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "LifestealEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "MinionEffect" DROP COLUMN "cardId";

-- AlterTable
ALTER TABLE "PhantomTouchEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "PickupEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "PoisonEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "ResilienceEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "RoarAuraEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "SpellEffect" DROP COLUMN "cardId";

-- AlterTable
ALTER TABLE "SprayOfKnivesEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "StunEffect" ADD COLUMN     "spellEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "TauntEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "TwinEffect" ADD COLUMN     "minionEffectId" TEXT NOT NULL;

-- DropTable
DROP TABLE "MinionEffectEffect";

-- DropTable
DROP TABLE "SpellEffectEffect";

-- CreateTable
CREATE TABLE "CardEffect" (
    "id" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,
    "spellEffectId" TEXT,
    "minionEffectId" TEXT,

    CONSTRAINT "CardEffect_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "EtherealEffect" ADD CONSTRAINT "EtherealEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "TwinEffect" ADD CONSTRAINT "TwinEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CleanseEffect" ADD CONSTRAINT "CleanseEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "BlockEffect" ADD CONSTRAINT "BlockEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "RoarAuraEffect" ADD CONSTRAINT "RoarAuraEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DyingWishHealEffect" ADD CONSTRAINT "DyingWishHealEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DyingWishDamageEffect" ADD CONSTRAINT "DyingWishDamageEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "TauntEffect" ADD CONSTRAINT "TauntEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ChargeEffect" ADD CONSTRAINT "ChargeEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "LifestealEffect" ADD CONSTRAINT "LifestealEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PickupEffect" ADD CONSTRAINT "PickupEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DamageEffect" ADD CONSTRAINT "DamageEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DazeEffect" ADD CONSTRAINT "DazeEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "BattleCryEffect" ADD CONSTRAINT "BattleCryEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CowardiceCurseEffect" ADD CONSTRAINT "CowardiceCurseEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PhantomTouchEffect" ADD CONSTRAINT "PhantomTouchEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SprayOfKnivesEffect" ADD CONSTRAINT "SprayOfKnivesEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HealEffect" ADD CONSTRAINT "HealEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ArmorEffect" ADD CONSTRAINT "ArmorEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ResilienceEffect" ADD CONSTRAINT "ResilienceEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PoisonEffect" ADD CONSTRAINT "PoisonEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "InitiativeEffect" ADD CONSTRAINT "InitiativeEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StunEffect" ADD CONSTRAINT "StunEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

/*
  Warnings:

  - You are about to drop the column `minionEffectId` on the `ChargeEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `LifestealEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `PickupEffect` table. All the data in the column will be lost.
  - You are about to drop the column `cardId` on the `SummonEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `SummonEffect` table. All the data in the column will be lost.
  - You are about to drop the column `minionEffectId` on the `TauntEffect` table. All the data in the column will be lost.

*/
-- DropForeignKey
ALTER TABLE "ChargeEffect" DROP CONSTRAINT "ChargeEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "LifestealEffect" DROP CONSTRAINT "LifestealEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "PickupEffect" DROP CONSTRAINT "PickupEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SummonEffect" DROP CONSTRAINT "SummonEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "TauntEffect" DROP CONSTRAINT "TauntEffect_minionEffectId_fkey";

-- DropIndex
DROP INDEX "ChargeEffect_minionEffectId_key";

-- DropIndex
DROP INDEX "LifestealEffect_minionEffectId_key";

-- DropIndex
DROP INDEX "PickupEffect_minionEffectId_key";

-- DropIndex
DROP INDEX "SummonEffect_minionEffectId_key";

-- DropIndex
DROP INDEX "TauntEffect_minionEffectId_key";

-- AlterTable
ALTER TABLE "ChargeEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "LifestealEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "PickupEffect" DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "SummonEffect" DROP COLUMN "cardId",
DROP COLUMN "minionEffectId";

-- AlterTable
ALTER TABLE "TauntEffect" DROP COLUMN "minionEffectId";

-- CreateTable
CREATE TABLE "MinionEffectEffect" (
    "id" TEXT NOT NULL,
    "minionEffectId" TEXT NOT NULL,
    "duration" INTEGER NOT NULL DEFAULT 0,
    "tauntEffectId" TEXT,
    "chargeEffectId" TEXT,
    "lifestealEffectId" TEXT,
    "pickupEffectId" TEXT,
    "summonEffectId" TEXT,
    "resilienceEffectId" TEXT,

    CONSTRAINT "MinionEffectEffect_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_tauntEffectId_fkey" FOREIGN KEY ("tauntEffectId") REFERENCES "TauntEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_chargeEffectId_fkey" FOREIGN KEY ("chargeEffectId") REFERENCES "ChargeEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_lifestealEffectId_fkey" FOREIGN KEY ("lifestealEffectId") REFERENCES "LifestealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_pickupEffectId_fkey" FOREIGN KEY ("pickupEffectId") REFERENCES "PickupEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_summonEffectId_fkey" FOREIGN KEY ("summonEffectId") REFERENCES "SummonEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_resilienceEffectId_fkey" FOREIGN KEY ("resilienceEffectId") REFERENCES "ResilienceEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

/*
  Warnings:

  - You are about to drop the `ChargeAbility` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `LifestealAbility` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `MinionAbility` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `PickupAbility` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `SummonAbility` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `TauntAbility` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "ChargeAbility" DROP CONSTRAINT "ChargeAbility_minionAbilityId_fkey";

-- DropForeignKey
ALTER TABLE "LifestealAbility" DROP CONSTRAINT "LifestealAbility_minionAbilityId_fkey";

-- DropForeignKey
ALTER TABLE "MinionAbility" DROP CONSTRAINT "MinionAbility_minionId_fkey";

-- DropForeignKey
ALTER TABLE "PickupAbility" DROP CONSTRAINT "PickupAbility_minionAbilityId_fkey";

-- DropForeignKey
ALTER TABLE "SummonAbility" DROP CONSTRAINT "SummonAbility_minionAbilityId_fkey";

-- DropForeignKey
ALTER TABLE "TauntAbility" DROP CONSTRAINT "TauntAbility_minionAbilityId_fkey";

-- AlterTable
ALTER TABLE "Minion" ADD COLUMN     "charge" BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN     "lifesteal" BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN     "pickupAmount" INTEGER,
ADD COLUMN     "summonCardId" TEXT,
ADD COLUMN     "taunt" BOOLEAN NOT NULL DEFAULT false;

-- DropTable
DROP TABLE "ChargeAbility";

-- DropTable
DROP TABLE "LifestealAbility";

-- DropTable
DROP TABLE "MinionAbility";

-- DropTable
DROP TABLE "PickupAbility";

-- DropTable
DROP TABLE "SummonAbility";

-- DropTable
DROP TABLE "TauntAbility";

-- DropEnum
DROP TYPE "AbilityType";

/*
  Warnings:

  - You are about to drop the column `poisonEffectId` on the `MinionEffectEffect` table. All the data in the column will be lost.
  - You are about to drop the column `resilienceEffectId` on the `MinionEffectEffect` table. All the data in the column will be lost.
  - You are about to drop the column `summonEffectId` on the `MinionEffectEffect` table. All the data in the column will be lost.
  - You are about to drop the `SummonEffect` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_poisonEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_resilienceEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_summonEffectId_fkey";

-- AlterTable
ALTER TABLE "MinionEffectEffect" DROP COLUMN "poisonEffectId",
DROP COLUMN "resilienceEffectId",
DROP COLUMN "summonEffectId",
ADD COLUMN     "blockEffectId" TEXT,
ADD COLUMN     "cleanseEffectId" TEXT,
ADD COLUMN     "dyingWishHealEffectId" TEXT,
ADD COLUMN     "etherealEffectId" TEXT,
ADD COLUMN     "lifeStealEffectId" TEXT,
ADD COLUMN     "roarAuraEffectId" TEXT,
ADD COLUMN     "twinEffectId" TEXT;

-- AlterTable
ALTER TABLE "SpellEffectEffect" ADD COLUMN     "battleCryEffectId" TEXT,
ADD COLUMN     "cowardiceCurseEffectId" TEXT,
ADD COLUMN     "phantomTouchEffectId" TEXT,
ADD COLUMN     "sprayOfKnivesEffectId" TEXT;

-- DropTable
DROP TABLE "SummonEffect";

-- CreateTable
CREATE TABLE "EtherealEffect" (
    "id" TEXT NOT NULL,

    CONSTRAINT "EtherealEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "TwinEffect" (
    "id" TEXT NOT NULL,

    CONSTRAINT "TwinEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "CleanseEffect" (
    "id" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "CleanseEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "BlockEffect" (
    "id" TEXT NOT NULL,

    CONSTRAINT "BlockEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "RoarAuraEffect" (
    "id" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "RoarAuraEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "DyingWishHealEffect" (
    "id" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "DyingWishHealEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "LifeStealEffect" (
    "id" TEXT NOT NULL,
    "percentage" INTEGER NOT NULL DEFAULT 100,

    CONSTRAINT "LifeStealEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "BattleCryEffect" (
    "id" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "BattleCryEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "CowardiceCurseEffect" (
    "id" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "CowardiceCurseEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PhantomTouchEffect" (
    "id" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "PhantomTouchEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "SprayOfKnivesEffect" (
    "id" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "SprayOfKnivesEffect_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_battleCryEffectId_fkey" FOREIGN KEY ("battleCryEffectId") REFERENCES "BattleCryEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_cowardiceCurseEffectId_fkey" FOREIGN KEY ("cowardiceCurseEffectId") REFERENCES "CowardiceCurseEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_phantomTouchEffectId_fkey" FOREIGN KEY ("phantomTouchEffectId") REFERENCES "PhantomTouchEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_sprayOfKnivesEffectId_fkey" FOREIGN KEY ("sprayOfKnivesEffectId") REFERENCES "SprayOfKnivesEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_etherealEffectId_fkey" FOREIGN KEY ("etherealEffectId") REFERENCES "EtherealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_twinEffectId_fkey" FOREIGN KEY ("twinEffectId") REFERENCES "TwinEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_cleanseEffectId_fkey" FOREIGN KEY ("cleanseEffectId") REFERENCES "CleanseEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_blockEffectId_fkey" FOREIGN KEY ("blockEffectId") REFERENCES "BlockEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_roarAuraEffectId_fkey" FOREIGN KEY ("roarAuraEffectId") REFERENCES "RoarAuraEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_dyingWishHealEffectId_fkey" FOREIGN KEY ("dyingWishHealEffectId") REFERENCES "DyingWishHealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_lifeStealEffectId_fkey" FOREIGN KEY ("lifeStealEffectId") REFERENCES "LifeStealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

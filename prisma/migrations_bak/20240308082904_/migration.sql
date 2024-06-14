/*
  Warnings:

  - You are about to drop the `Minion` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "Minion" DROP CONSTRAINT "Minion_cardId_fkey";

-- DropTable
DROP TABLE "Minion";

-- DropEnum
DROP TYPE "EffectType";

-- CreateTable
CREATE TABLE "MinionEffect" (
    "id" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,
    "duration" INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT "MinionEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "TauntEffect" (
    "id" TEXT NOT NULL,
    "minionEffectId" TEXT NOT NULL,

    CONSTRAINT "TauntEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ChargeEffect" (
    "id" TEXT NOT NULL,
    "minionEffectId" TEXT NOT NULL,

    CONSTRAINT "ChargeEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "LifestealEffect" (
    "id" TEXT NOT NULL,
    "minionEffectId" TEXT NOT NULL,

    CONSTRAINT "LifestealEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PickupEffect" (
    "id" TEXT NOT NULL,
    "minionEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "PickupEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "SummonEffect" (
    "id" TEXT NOT NULL,
    "minionEffectId" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,

    CONSTRAINT "SummonEffect_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "TauntEffect_minionEffectId_key" ON "TauntEffect"("minionEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "ChargeEffect_minionEffectId_key" ON "ChargeEffect"("minionEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "LifestealEffect_minionEffectId_key" ON "LifestealEffect"("minionEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "PickupEffect_minionEffectId_key" ON "PickupEffect"("minionEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "SummonEffect_minionEffectId_key" ON "SummonEffect"("minionEffectId");

-- AddForeignKey
ALTER TABLE "MinionEffect" ADD CONSTRAINT "MinionEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "TauntEffect" ADD CONSTRAINT "TauntEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ChargeEffect" ADD CONSTRAINT "ChargeEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "LifestealEffect" ADD CONSTRAINT "LifestealEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PickupEffect" ADD CONSTRAINT "PickupEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SummonEffect" ADD CONSTRAINT "SummonEffect_minionEffectId_fkey" FOREIGN KEY ("minionEffectId") REFERENCES "MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

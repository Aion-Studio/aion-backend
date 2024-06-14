/*
  Warnings:

  - You are about to drop the `SpellEffect` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_cardId_fkey";

-- DropTable
DROP TABLE "SpellEffect";

-- CreateTable
CREATE TABLE "CardEffect" (
    "id" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,
    "duration" INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT "CardEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "DamageEffect" (
    "id" TEXT NOT NULL,
    "cardEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,
    "damageType" "DamageType" NOT NULL,
    "targetType" "TargetType" NOT NULL,

    CONSTRAINT "DamageEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HealEffect" (
    "id" TEXT NOT NULL,
    "cardEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,
    "targetType" "TargetType" NOT NULL,

    CONSTRAINT "HealEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ArmorEffect" (
    "id" TEXT NOT NULL,
    "cardEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "ArmorEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ResilienceEffect" (
    "id" TEXT NOT NULL,
    "cardEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "ResilienceEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PoisonEffect" (
    "id" TEXT NOT NULL,
    "cardEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "PoisonEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "InitiativeEffect" (
    "id" TEXT NOT NULL,
    "cardEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "InitiativeEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "StunEffect" (
    "id" TEXT NOT NULL,
    "cardEffectId" TEXT NOT NULL,

    CONSTRAINT "StunEffect_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "DamageEffect_cardEffectId_key" ON "DamageEffect"("cardEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "HealEffect_cardEffectId_key" ON "HealEffect"("cardEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "ArmorEffect_cardEffectId_key" ON "ArmorEffect"("cardEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "ResilienceEffect_cardEffectId_key" ON "ResilienceEffect"("cardEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "PoisonEffect_cardEffectId_key" ON "PoisonEffect"("cardEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "InitiativeEffect_cardEffectId_key" ON "InitiativeEffect"("cardEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "StunEffect_cardEffectId_key" ON "StunEffect"("cardEffectId");

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DamageEffect" ADD CONSTRAINT "DamageEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "CardEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HealEffect" ADD CONSTRAINT "HealEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "CardEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ArmorEffect" ADD CONSTRAINT "ArmorEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "CardEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ResilienceEffect" ADD CONSTRAINT "ResilienceEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "CardEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PoisonEffect" ADD CONSTRAINT "PoisonEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "CardEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "InitiativeEffect" ADD CONSTRAINT "InitiativeEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "CardEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "StunEffect" ADD CONSTRAINT "StunEffect_cardEffectId_fkey" FOREIGN KEY ("cardEffectId") REFERENCES "CardEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

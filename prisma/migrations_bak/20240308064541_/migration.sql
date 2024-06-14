/*
  Warnings:

  - You are about to drop the `CardEffect` table. If the table is not empty, all the data it contains will be lost.
  - Added the required column `cardType` to the `Card` table without a default value. This is not possible if the table is not empty.

*/
-- CreateEnum
CREATE TYPE "CardType" AS ENUM ('Spell', 'Minion');

-- CreateEnum
CREATE TYPE "EffectType" AS ENUM ('Damage', 'Heal', 'Armor', 'Resilience', 'Poison', 'Initiative', 'Stun');

-- CreateEnum
CREATE TYPE "TargetType" AS ENUM ('Minion', 'Hero', 'Both');

-- CreateEnum
CREATE TYPE "AbilityType" AS ENUM ('Taunt', 'Charge', 'Lifesteal', 'Pickup', 'Summon');

-- DropForeignKey
ALTER TABLE "CardEffect" DROP CONSTRAINT "CardEffect_cardId_fkey";

-- AlterTable
ALTER TABLE "Card" ADD COLUMN     "cardType" "CardType" NOT NULL;

-- DropTable
DROP TABLE "CardEffect";

-- DropEnum
DROP TYPE "EffectEnum";

-- CreateTable
CREATE TABLE "SpellEffect" (
    "id" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,
    "duration" INTEGER NOT NULL DEFAULT 0,
    "effectType" "EffectType" NOT NULL,

    CONSTRAINT "SpellEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Minion" (
    "id" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,
    "health" INTEGER NOT NULL,
    "damage" INTEGER NOT NULL,

    CONSTRAINT "Minion_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "MinionAbility" (
    "id" TEXT NOT NULL,
    "minionId" TEXT NOT NULL,
    "abilityType" "AbilityType" NOT NULL,

    CONSTRAINT "MinionAbility_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "DamageEffect" (
    "id" TEXT NOT NULL,
    "spellEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,
    "damageType" "DamageType" NOT NULL,
    "targetType" "TargetType" NOT NULL,

    CONSTRAINT "DamageEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HealEffect" (
    "id" TEXT NOT NULL,
    "spellEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,
    "targetType" "TargetType" NOT NULL,

    CONSTRAINT "HealEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ArmorEffect" (
    "id" TEXT NOT NULL,
    "spellEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "ArmorEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ResilienceEffect" (
    "id" TEXT NOT NULL,
    "spellEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "ResilienceEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PoisonEffect" (
    "id" TEXT NOT NULL,
    "spellEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "PoisonEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "InitiativeEffect" (
    "id" TEXT NOT NULL,
    "spellEffectId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "InitiativeEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "StunEffect" (
    "id" TEXT NOT NULL,
    "spellEffectId" TEXT NOT NULL,

    CONSTRAINT "StunEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "TauntAbility" (
    "id" TEXT NOT NULL,
    "minionAbilityId" TEXT NOT NULL,

    CONSTRAINT "TauntAbility_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ChargeAbility" (
    "id" TEXT NOT NULL,
    "minionAbilityId" TEXT NOT NULL,

    CONSTRAINT "ChargeAbility_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "LifestealAbility" (
    "id" TEXT NOT NULL,
    "minionAbilityId" TEXT NOT NULL,

    CONSTRAINT "LifestealAbility_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "PickupAbility" (
    "id" TEXT NOT NULL,
    "minionAbilityId" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "PickupAbility_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "SummonAbility" (
    "id" TEXT NOT NULL,
    "minionAbilityId" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,

    CONSTRAINT "SummonAbility_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "SpellEffect_cardId_key" ON "SpellEffect"("cardId");

-- CreateIndex
CREATE UNIQUE INDEX "Minion_cardId_key" ON "Minion"("cardId");

-- CreateIndex
CREATE UNIQUE INDEX "DamageEffect_spellEffectId_key" ON "DamageEffect"("spellEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "HealEffect_spellEffectId_key" ON "HealEffect"("spellEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "ArmorEffect_spellEffectId_key" ON "ArmorEffect"("spellEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "ResilienceEffect_spellEffectId_key" ON "ResilienceEffect"("spellEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "PoisonEffect_spellEffectId_key" ON "PoisonEffect"("spellEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "InitiativeEffect_spellEffectId_key" ON "InitiativeEffect"("spellEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "StunEffect_spellEffectId_key" ON "StunEffect"("spellEffectId");

-- CreateIndex
CREATE UNIQUE INDEX "TauntAbility_minionAbilityId_key" ON "TauntAbility"("minionAbilityId");

-- CreateIndex
CREATE UNIQUE INDEX "ChargeAbility_minionAbilityId_key" ON "ChargeAbility"("minionAbilityId");

-- CreateIndex
CREATE UNIQUE INDEX "LifestealAbility_minionAbilityId_key" ON "LifestealAbility"("minionAbilityId");

-- CreateIndex
CREATE UNIQUE INDEX "PickupAbility_minionAbilityId_key" ON "PickupAbility"("minionAbilityId");

-- CreateIndex
CREATE UNIQUE INDEX "SummonAbility_minionAbilityId_key" ON "SummonAbility"("minionAbilityId");

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Minion" ADD CONSTRAINT "Minion_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MinionAbility" ADD CONSTRAINT "MinionAbility_minionId_fkey" FOREIGN KEY ("minionId") REFERENCES "Minion"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DamageEffect" ADD CONSTRAINT "DamageEffect_spellEffectId_fkey" FOREIGN KEY ("spellEffectId") REFERENCES "SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

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

-- AddForeignKey
ALTER TABLE "TauntAbility" ADD CONSTRAINT "TauntAbility_minionAbilityId_fkey" FOREIGN KEY ("minionAbilityId") REFERENCES "MinionAbility"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ChargeAbility" ADD CONSTRAINT "ChargeAbility_minionAbilityId_fkey" FOREIGN KEY ("minionAbilityId") REFERENCES "MinionAbility"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "LifestealAbility" ADD CONSTRAINT "LifestealAbility_minionAbilityId_fkey" FOREIGN KEY ("minionAbilityId") REFERENCES "MinionAbility"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PickupAbility" ADD CONSTRAINT "PickupAbility_minionAbilityId_fkey" FOREIGN KEY ("minionAbilityId") REFERENCES "MinionAbility"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SummonAbility" ADD CONSTRAINT "SummonAbility_minionAbilityId_fkey" FOREIGN KEY ("minionAbilityId") REFERENCES "MinionAbility"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

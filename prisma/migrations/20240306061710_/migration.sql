/*
  Warnings:

  - A unique constraint covering the columns `[deckId]` on the table `Hero` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateEnum
CREATE TYPE "Nation" AS ENUM ('Dusane', 'Aylen', 'Ironmark', 'Kelidor', 'Meta');

-- CreateEnum
CREATE TYPE "Rarity" AS ENUM ('Common', 'Magic', 'Epic', 'Legendary');

-- CreateEnum
CREATE TYPE "EffectType" AS ENUM ('PhysicalDamage', 'SpellDamage', 'ChaosDamage', 'DamageOverTime', 'Stun', 'ReduceArmor', 'ReduceResilience', 'IncreaseArmor', 'IncreaseResilience', 'Heal', 'HealOverTime', 'DrawCards', 'ApplyPoison', 'RemovePoison', 'ApplyInitiative', 'RemoveInitiative');

-- AlterTable
ALTER TABLE "Hero" ADD COLUMN     "deckId" TEXT;

-- CreateTable
CREATE TABLE "Deck" (
    "id" TEXT NOT NULL,
    "heroId" TEXT,

    CONSTRAINT "Deck_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Card" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "nation" "Nation" NOT NULL,
    "rarity" "Rarity" NOT NULL,
    "tier" INTEGER NOT NULL,
    "heroId" TEXT NOT NULL,

    CONSTRAINT "Card_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "CardInDeck" (
    "deckId" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,
    "quantity" INTEGER NOT NULL DEFAULT 1,

    CONSTRAINT "CardInDeck_pkey" PRIMARY KEY ("deckId","cardId")
);

-- CreateTable
CREATE TABLE "CardEffect" (
    "id" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,
    "effect" "EffectType" NOT NULL,
    "value" INTEGER,
    "duration" INTEGER,

    CONSTRAINT "CardEffect_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Deck_heroId_key" ON "Deck"("heroId");

-- CreateIndex
CREATE UNIQUE INDEX "CardEffect_cardId_effect_key" ON "CardEffect"("cardId", "effect");

-- CreateIndex
CREATE UNIQUE INDEX "Hero_deckId_key" ON "Hero"("deckId");

-- AddForeignKey
ALTER TABLE "Hero" ADD CONSTRAINT "Hero_deckId_fkey" FOREIGN KEY ("deckId") REFERENCES "Deck"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Card" ADD CONSTRAINT "Card_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CardInDeck" ADD CONSTRAINT "CardInDeck_deckId_fkey" FOREIGN KEY ("deckId") REFERENCES "Deck"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CardInDeck" ADD CONSTRAINT "CardInDeck_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

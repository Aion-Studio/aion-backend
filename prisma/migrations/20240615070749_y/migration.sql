-- CreateEnum
CREATE TYPE "Class" AS ENUM ('Fighter', 'Ranger', 'Wizard');

-- CreateEnum
CREATE TYPE "EffectType" AS ENUM ('Heal', 'Damage', 'Buff', 'Debuff');

-- CreateEnum
CREATE TYPE "Resource" AS ENUM ('Aion', 'Flux', 'Gem');

-- CreateEnum
CREATE TYPE "TargetType" AS ENUM ('Opponent', 'Itself');

-- CreateEnum
CREATE TYPE "Nation" AS ENUM ('Dusane', 'Aylen', 'Ironmark', 'Kelidor', 'Meta');

-- CreateEnum
CREATE TYPE "Rarity" AS ENUM ('Common', 'Rare', 'Epic', 'Legendary');

-- CreateTable
CREATE TABLE "Account" (
    "id" TEXT NOT NULL,
    "supabase_user_id" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Account_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Hero" (
    "id" TEXT NOT NULL,
    "class" "Class" NOT NULL,
    "hp" INTEGER NOT NULL,
    "strength" INTEGER NOT NULL,
    "dexterity" INTEGER NOT NULL,
    "intelligence" INTEGER NOT NULL,
    "explore" INTEGER NOT NULL,
    "stamina" INTEGER NOT NULL,
    "crafting" INTEGER NOT NULL,
    "accountId" TEXT,

    CONSTRAINT "Hero_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Relic" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "heroId" TEXT NOT NULL,

    CONSTRAINT "Relic_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "RelicEffect" (
    "id" TEXT NOT NULL,
    "type" "EffectType" NOT NULL,
    "value" INTEGER NOT NULL,
    "target" "TargetType" NOT NULL,
    "duration" INTEGER,
    "relicId" TEXT NOT NULL,

    CONSTRAINT "RelicEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HeroSpell" (
    "id" TEXT NOT NULL,
    "slot" INTEGER NOT NULL,
    "heroId" TEXT NOT NULL,
    "spellId" TEXT NOT NULL,

    CONSTRAINT "HeroSpell_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Spell" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    "duration" INTEGER,

    CONSTRAINT "Spell_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "SpellEffect" (
    "id" TEXT NOT NULL,
    "value" INTEGER NOT NULL,
    "target" "TargetType" NOT NULL,
    "effect" "EffectType" NOT NULL,
    "spellId" TEXT,

    CONSTRAINT "SpellEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HeroResource" (
    "id" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,
    "resource_type_id" TEXT NOT NULL,

    CONSTRAINT "HeroResource_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Action" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT,
    "region_name" TEXT NOT NULL,
    "leyline_id" TEXT,
    "quest_id" TEXT,
    "npc_id" TEXT,
    "cost" INTEGER DEFAULT 0,

    CONSTRAINT "Action_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HeroActions" (
    "id" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "action_id" TEXT NOT NULL,

    CONSTRAINT "HeroActions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Quest" (
    "id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "region_name" TEXT NOT NULL,
    "questNumber" INTEGER NOT NULL,
    "cost" INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT "Quest_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HeroQuests" (
    "id" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "quest_id" TEXT NOT NULL,
    "accepted" BOOLEAN NOT NULL DEFAULT false,
    "completed" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "HeroQuests_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ActionCompleted" (
    "id" TEXT NOT NULL,
    "action_name" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "completed_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "lootBox" JSONB NOT NULL DEFAULT '{}',

    CONSTRAINT "ActionCompleted_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HeroRegion" (
    "id" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "discovery_level" DOUBLE PRECISION NOT NULL,
    "region_name" TEXT NOT NULL,
    "current_location" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "HeroRegion_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ResourceType" (
    "id" TEXT NOT NULL,
    "type" "Resource" NOT NULL,

    CONSTRAINT "ResourceType_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ResourceCost" (
    "id" TEXT NOT NULL,
    "resource" "Resource" NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "ResourceCost_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Region" (
    "name" TEXT NOT NULL,
    "adjacent_regions" TEXT[],

    CONSTRAINT "Region_pkey" PRIMARY KEY ("name")
);

-- CreateTable
CREATE TABLE "Leyline" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "xp_reward" INTEGER NOT NULL,
    "RegionName" TEXT NOT NULL,
    "discovery_required" INTEGER NOT NULL,
    "stamina_rate" DOUBLE PRECISION NOT NULL,
    "aion_rate" DOUBLE PRECISION NOT NULL,
    "npcId" TEXT NOT NULL,

    CONSTRAINT "Leyline_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Npc" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    "armor" INTEGER NOT NULL DEFAULT 0,
    "strength" INTEGER NOT NULL,
    "damageMin" INTEGER NOT NULL DEFAULT 0,
    "damageMax" INTEGER NOT NULL DEFAULT 0,
    "hp" INTEGER NOT NULL DEFAULT 150,

    CONSTRAINT "Npc_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Card" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "class" "Class" NOT NULL,
    "cost" INTEGER NOT NULL,
    "imgUrl" TEXT NOT NULL,

    CONSTRAINT "Card_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "CardEffect" (
    "id" TEXT NOT NULL,
    "efffectType" "EffectType" NOT NULL,
    "value" INTEGER NOT NULL,
    "target" "TargetType" NOT NULL,
    "duration" INTEGER,
    "cardId" TEXT NOT NULL,

    CONSTRAINT "CardEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HeroCard" (
    "id" TEXT NOT NULL,
    "heroId" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,

    CONSTRAINT "HeroCard_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "DeckCard" (
    "id" TEXT NOT NULL,
    "deckId" TEXT NOT NULL,
    "heroCardId" TEXT,

    CONSTRAINT "DeckCard_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Deck" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL DEFAULT 'my deck',
    "heroId" TEXT,
    "active" BOOLEAN NOT NULL DEFAULT false,
    "npcId" TEXT,

    CONSTRAINT "Deck_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "_QuestRequirements" (
    "A" TEXT NOT NULL,
    "B" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Account_supabase_user_id_key" ON "Account"("supabase_user_id");

-- CreateIndex
CREATE UNIQUE INDEX "HeroResource_hero_id_resource_type_id_key" ON "HeroResource"("hero_id", "resource_type_id");

-- CreateIndex
CREATE UNIQUE INDEX "Action_name_description_quest_id_key" ON "Action"("name", "description", "quest_id");

-- CreateIndex
CREATE UNIQUE INDEX "ResourceType_type_key" ON "ResourceType"("type");

-- CreateIndex
CREATE UNIQUE INDEX "Leyline_npcId_key" ON "Leyline"("npcId");

-- CreateIndex
CREATE UNIQUE INDEX "_QuestRequirements_AB_unique" ON "_QuestRequirements"("A", "B");

-- CreateIndex
CREATE INDEX "_QuestRequirements_B_index" ON "_QuestRequirements"("B");

-- AddForeignKey
ALTER TABLE "Hero" ADD CONSTRAINT "Hero_accountId_fkey" FOREIGN KEY ("accountId") REFERENCES "Account"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Relic" ADD CONSTRAINT "Relic_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "RelicEffect" ADD CONSTRAINT "RelicEffect_relicId_fkey" FOREIGN KEY ("relicId") REFERENCES "Relic"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroSpell" ADD CONSTRAINT "HeroSpell_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroSpell" ADD CONSTRAINT "HeroSpell_spellId_fkey" FOREIGN KEY ("spellId") REFERENCES "Spell"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_spellId_fkey" FOREIGN KEY ("spellId") REFERENCES "Spell"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroResource" ADD CONSTRAINT "HeroResource_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroResource" ADD CONSTRAINT "HeroResource_resource_type_id_fkey" FOREIGN KEY ("resource_type_id") REFERENCES "ResourceType"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Action" ADD CONSTRAINT "Action_region_name_fkey" FOREIGN KEY ("region_name") REFERENCES "Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Action" ADD CONSTRAINT "Action_leyline_id_fkey" FOREIGN KEY ("leyline_id") REFERENCES "Leyline"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Action" ADD CONSTRAINT "Action_quest_id_fkey" FOREIGN KEY ("quest_id") REFERENCES "Quest"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Action" ADD CONSTRAINT "Action_npc_id_fkey" FOREIGN KEY ("npc_id") REFERENCES "Npc"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroActions" ADD CONSTRAINT "HeroActions_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroActions" ADD CONSTRAINT "HeroActions_action_id_fkey" FOREIGN KEY ("action_id") REFERENCES "Action"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Quest" ADD CONSTRAINT "Quest_region_name_fkey" FOREIGN KEY ("region_name") REFERENCES "Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroQuests" ADD CONSTRAINT "HeroQuests_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroQuests" ADD CONSTRAINT "HeroQuests_quest_id_fkey" FOREIGN KEY ("quest_id") REFERENCES "Quest"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ActionCompleted" ADD CONSTRAINT "ActionCompleted_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroRegion" ADD CONSTRAINT "HeroRegion_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroRegion" ADD CONSTRAINT "HeroRegion_region_name_fkey" FOREIGN KEY ("region_name") REFERENCES "Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Leyline" ADD CONSTRAINT "Leyline_RegionName_fkey" FOREIGN KEY ("RegionName") REFERENCES "Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Leyline" ADD CONSTRAINT "Leyline_npcId_fkey" FOREIGN KEY ("npcId") REFERENCES "Npc"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroCard" ADD CONSTRAINT "HeroCard_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroCard" ADD CONSTRAINT "HeroCard_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DeckCard" ADD CONSTRAINT "DeckCard_heroCardId_fkey" FOREIGN KEY ("heroCardId") REFERENCES "HeroCard"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Deck" ADD CONSTRAINT "Deck_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Deck" ADD CONSTRAINT "Deck_npcId_fkey" FOREIGN KEY ("npcId") REFERENCES "Npc"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_QuestRequirements" ADD CONSTRAINT "_QuestRequirements_A_fkey" FOREIGN KEY ("A") REFERENCES "Quest"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_QuestRequirements" ADD CONSTRAINT "_QuestRequirements_B_fkey" FOREIGN KEY ("B") REFERENCES "Quest"("id") ON DELETE CASCADE ON UPDATE CASCADE;

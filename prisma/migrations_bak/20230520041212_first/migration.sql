-- CreateEnum
CREATE TYPE "ResourceType" AS ENUM ('Aion', 'Valor', 'NexusShard');

-- CreateEnum
CREATE TYPE "MaterialType" AS ENUM ('Oak', 'IronOre', 'Copper', 'Silk');

-- CreateTable
CREATE TABLE "Item" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "weight" INTEGER NOT NULL,
    "value" INTEGER NOT NULL,
    "active_inventory_id" TEXT,
    "backpack_inventory_id" TEXT,

    CONSTRAINT "Item_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Inventory" (
    "id" TEXT NOT NULL,

    CONSTRAINT "Inventory_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Hero" (
    "id" TEXT NOT NULL,
    "aionCapacity" INTEGER NOT NULL,
    "aionCollected" INTEGER NOT NULL,
    "base_stats_id" TEXT NOT NULL,
    "attributes_id" TEXT NOT NULL,
    "inventory_id" TEXT NOT NULL,

    CONSTRAINT "Hero_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HeroRegion" (
    "id" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "region_name" TEXT NOT NULL,
    "discovery_level" INTEGER NOT NULL,

    CONSTRAINT "HeroRegion_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "BaseStats" (
    "id" TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    "xp" INTEGER NOT NULL,
    "damageMin" INTEGER NOT NULL,
    "damageMax" INTEGER NOT NULL,
    "hit_points" INTEGER NOT NULL,
    "mana" INTEGER NOT NULL,
    "armor" INTEGER NOT NULL,

    CONSTRAINT "BaseStats_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Attributes" (
    "id" TEXT NOT NULL,
    "resilience" INTEGER NOT NULL,
    "strength" INTEGER NOT NULL,
    "agility" INTEGER NOT NULL,
    "intelligence" INTEGER NOT NULL,
    "exploration" INTEGER NOT NULL,
    "crafting" INTEGER NOT NULL,

    CONSTRAINT "Attributes_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "RetinueSlot" (
    "id" TEXT NOT NULL,
    "slotType" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "followerId" TEXT,

    CONSTRAINT "RetinueSlot_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Follower" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    "attributes_id" TEXT NOT NULL,

    CONSTRAINT "Follower_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ResourceCost" (
    "id" TEXT NOT NULL,
    "resource" "ResourceType" NOT NULL,
    "amount" INTEGER NOT NULL,
    "material" "MaterialType" NOT NULL,

    CONSTRAINT "ResourceCost_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Region" (
    "name" TEXT NOT NULL,
    "adjacent_regions" TEXT NOT NULL,

    CONSTRAINT "Region_pkey" PRIMARY KEY ("name")
);

-- CreateTable
CREATE TABLE "Leyline" (
    "id" TEXT NOT NULL,
    "location" TEXT NOT NULL,
    "xp_reward" INTEGER NOT NULL,
    "RegionName" TEXT NOT NULL,

    CONSTRAINT "Leyline_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Hero_base_stats_id_key" ON "Hero"("base_stats_id");

-- CreateIndex
CREATE UNIQUE INDEX "Hero_attributes_id_key" ON "Hero"("attributes_id");

-- CreateIndex
CREATE UNIQUE INDEX "Hero_inventory_id_key" ON "Hero"("inventory_id");

-- AddForeignKey
ALTER TABLE "Item" ADD CONSTRAINT "Item_active_inventory_id_fkey" FOREIGN KEY ("active_inventory_id") REFERENCES "Inventory"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Item" ADD CONSTRAINT "Item_backpack_inventory_id_fkey" FOREIGN KEY ("backpack_inventory_id") REFERENCES "Inventory"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Hero" ADD CONSTRAINT "Hero_base_stats_id_fkey" FOREIGN KEY ("base_stats_id") REFERENCES "BaseStats"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Hero" ADD CONSTRAINT "Hero_attributes_id_fkey" FOREIGN KEY ("attributes_id") REFERENCES "Attributes"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Hero" ADD CONSTRAINT "Hero_inventory_id_fkey" FOREIGN KEY ("inventory_id") REFERENCES "Inventory"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroRegion" ADD CONSTRAINT "HeroRegion_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroRegion" ADD CONSTRAINT "HeroRegion_region_name_fkey" FOREIGN KEY ("region_name") REFERENCES "Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "RetinueSlot" ADD CONSTRAINT "RetinueSlot_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "RetinueSlot" ADD CONSTRAINT "RetinueSlot_followerId_fkey" FOREIGN KEY ("followerId") REFERENCES "Follower"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Follower" ADD CONSTRAINT "Follower_attributes_id_fkey" FOREIGN KEY ("attributes_id") REFERENCES "Attributes"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Leyline" ADD CONSTRAINT "Leyline_RegionName_fkey" FOREIGN KEY ("RegionName") REFERENCES "Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;

/*
  Warnings:

  - You are about to drop the column `material` on the `ResourceCost` table. All the data in the column will be lost.

*/
-- AlterEnum
-- This migration adds more than one value to an enum.
-- With PostgreSQL versions 11 and earlier, this is not possible
-- in a single migration. This can be worked around by creating
-- multiple migrations, each migration adding only one value to
-- the enum.


ALTER TYPE "ResourceType" ADD VALUE 'Oak';
ALTER TYPE "ResourceType" ADD VALUE 'IronOre';
ALTER TYPE "ResourceType" ADD VALUE 'Copper';
ALTER TYPE "ResourceType" ADD VALUE 'Silk';

-- AlterTable
ALTER TABLE "ResourceCost" DROP COLUMN "material";

-- DropEnum
DROP TYPE "MaterialType";

-- CreateTable
CREATE TABLE "ExploreAction" (
    "id" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "duration" INTEGER NOT NULL,
    "region_name" TEXT NOT NULL,
    "xp" INTEGER NOT NULL,
    "discovery_level" INTEGER NOT NULL,
    "start_time" TIMESTAMP(3),

    CONSTRAINT "ExploreAction_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ResourceValue" (
    "id" TEXT NOT NULL,
    "resource" "ResourceType" NOT NULL,
    "value" INTEGER NOT NULL,
    "regionActionResultId" TEXT NOT NULL,

    CONSTRAINT "ResourceValue_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "RegionActionResult" (
    "id" TEXT NOT NULL,
    "xp" INTEGER NOT NULL,
    "discovery_level_increase" DOUBLE PRECISION NOT NULL,
    "hero_id" TEXT NOT NULL,

    CONSTRAINT "RegionActionResult_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "ExploreAction" ADD CONSTRAINT "ExploreAction_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ExploreAction" ADD CONSTRAINT "ExploreAction_region_name_fkey" FOREIGN KEY ("region_name") REFERENCES "Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ResourceValue" ADD CONSTRAINT "ResourceValue_regionActionResultId_fkey" FOREIGN KEY ("regionActionResultId") REFERENCES "RegionActionResult"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "RegionActionResult" ADD CONSTRAINT "RegionActionResult_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

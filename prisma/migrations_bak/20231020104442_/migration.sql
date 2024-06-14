/*
  Warnings:

  - You are about to drop the column `resource` on the `HeroResource` table. All the data in the column will be lost.
  - Added the required column `resourceTypeId` to the `HeroResource` table without a default value. This is not possible if the table is not empty.
  - Changed the type of `resource` on the `ResourceCost` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- CreateEnum
CREATE TYPE "ResourceEnum" AS ENUM ('Aion', 'Valor', 'NexusShard', 'Material');

-- CreateEnum
CREATE TYPE "MaterialEnum" AS ENUM ('Common', 'Rare', 'Epic');

-- CreateEnum
CREATE TYPE "CommonEnum" AS ENUM ('IronOre', 'RoughLeather', 'Quartz');

-- CreateEnum
CREATE TYPE "RareEnum" AS ENUM ('SilverOre', 'FineLeather', 'Sapphire');

-- CreateEnum
CREATE TYPE "EpicEnum" AS ENUM ('MythrilOre', 'Dragonhide', 'Ruby');

-- AlterTable
ALTER TABLE "HeroResource" DROP COLUMN "resource",
ADD COLUMN     "resourceTypeId" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "ResourceCost" DROP COLUMN "resource",
ADD COLUMN     "resource" "ResourceEnum" NOT NULL;

-- DropEnum
DROP TYPE "ResourceType";

-- CreateTable
CREATE TABLE "ResourceType" (
    "id" TEXT NOT NULL,
    "type" "ResourceEnum" NOT NULL,
    "materialTypeId" TEXT,

    CONSTRAINT "ResourceType_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "MaterialType" (
    "id" TEXT NOT NULL,
    "type" "MaterialEnum" NOT NULL,
    "commonId" TEXT,
    "rareId" TEXT,
    "epicId" TEXT,

    CONSTRAINT "MaterialType_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Common" (
    "id" TEXT NOT NULL,
    "type" "CommonEnum" NOT NULL,

    CONSTRAINT "Common_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Rare" (
    "id" TEXT NOT NULL,
    "type" "RareEnum" NOT NULL,

    CONSTRAINT "Rare_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Epic" (
    "id" TEXT NOT NULL,
    "type" "EpicEnum" NOT NULL,

    CONSTRAINT "Epic_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "ResourceType_materialTypeId_key" ON "ResourceType"("materialTypeId");

-- CreateIndex
CREATE UNIQUE INDEX "MaterialType_commonId_key" ON "MaterialType"("commonId");

-- CreateIndex
CREATE UNIQUE INDEX "MaterialType_rareId_key" ON "MaterialType"("rareId");

-- CreateIndex
CREATE UNIQUE INDEX "MaterialType_epicId_key" ON "MaterialType"("epicId");

-- AddForeignKey
ALTER TABLE "HeroResource" ADD CONSTRAINT "HeroResource_resourceTypeId_fkey" FOREIGN KEY ("resourceTypeId") REFERENCES "ResourceType"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ResourceType" ADD CONSTRAINT "ResourceType_materialTypeId_fkey" FOREIGN KEY ("materialTypeId") REFERENCES "MaterialType"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MaterialType" ADD CONSTRAINT "MaterialType_commonId_fkey" FOREIGN KEY ("commonId") REFERENCES "Common"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MaterialType" ADD CONSTRAINT "MaterialType_rareId_fkey" FOREIGN KEY ("rareId") REFERENCES "Rare"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "MaterialType" ADD CONSTRAINT "MaterialType_epicId_fkey" FOREIGN KEY ("epicId") REFERENCES "Epic"("id") ON DELETE SET NULL ON UPDATE CASCADE;

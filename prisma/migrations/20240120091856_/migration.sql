/*
  Warnings:

  - The values [NexusShard,Material] on the enum `ResourceEnum` will be removed. If these variants are still used in the database, this will fail.
  - You are about to drop the column `mana` on the `BaseStats` table. All the data in the column will be lost.
  - You are about to drop the column `materialTypeId` on the `ResourceType` table. All the data in the column will be lost.
  - You are about to drop the `Common` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `Epic` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `MaterialType` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `Rare` table. If the table is not empty, all the data it contains will be lost.

*/
-- AlterEnum
BEGIN;
CREATE TYPE "ResourceEnum_new" AS ENUM ('Aion', 'Valor', 'NexusOrb', 'StormShard');
ALTER TABLE "ResourceType" ALTER COLUMN "type" TYPE "ResourceEnum_new" USING ("type"::text::"ResourceEnum_new");
ALTER TABLE "ResourceCost" ALTER COLUMN "resource" TYPE "ResourceEnum_new" USING ("resource"::text::"ResourceEnum_new");
ALTER TYPE "ResourceEnum" RENAME TO "ResourceEnum_old";
ALTER TYPE "ResourceEnum_new" RENAME TO "ResourceEnum";
DROP TYPE "ResourceEnum_old";
COMMIT;

-- DropForeignKey
ALTER TABLE "MaterialType" DROP CONSTRAINT "MaterialType_commonId_fkey";

-- DropForeignKey
ALTER TABLE "MaterialType" DROP CONSTRAINT "MaterialType_epicId_fkey";

-- DropForeignKey
ALTER TABLE "MaterialType" DROP CONSTRAINT "MaterialType_rareId_fkey";

-- DropForeignKey
ALTER TABLE "ResourceType" DROP CONSTRAINT "ResourceType_materialTypeId_fkey";

-- DropIndex
DROP INDEX "ResourceType_materialTypeId_key";

-- AlterTable
ALTER TABLE "BaseStats" DROP COLUMN "mana";

-- AlterTable
ALTER TABLE "ResourceType" DROP COLUMN "materialTypeId";

-- DropTable
DROP TABLE "Common";

-- DropTable
DROP TABLE "Epic";

-- DropTable
DROP TABLE "MaterialType";

-- DropTable
DROP TABLE "Rare";

-- DropEnum
DROP TYPE "CommonEnum";

-- DropEnum
DROP TYPE "EpicEnum";

-- DropEnum
DROP TYPE "MaterialEnum";

-- DropEnum
DROP TYPE "RareEnum";

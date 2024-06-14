/*
  Warnings:

  - You are about to drop the `ExploreAction` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "ExploreAction" DROP CONSTRAINT "ExploreAction_hero_id_fkey";

-- DropForeignKey
ALTER TABLE "ExploreAction" DROP CONSTRAINT "ExploreAction_region_name_fkey";

-- AlterTable
ALTER TABLE "Hero" ADD COLUMN     "name" TEXT NOT NULL DEFAULT 'Default name';

-- DropTable
DROP TABLE "ExploreAction";

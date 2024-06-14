/*
  Warnings:

  - The `adjacent_regions` column on the `Region` table would be dropped and recreated. This will lead to data loss if there is data in the column.

*/
-- AlterTable
ALTER TABLE "HeroRegion" ADD COLUMN     "current_location" BOOLEAN NOT NULL DEFAULT false;

-- AlterTable
ALTER TABLE "Region" DROP COLUMN "adjacent_regions",
ADD COLUMN     "adjacent_regions" TEXT[];

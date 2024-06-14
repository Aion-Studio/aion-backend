/*
  Warnings:

  - Added the required column `stamina_cost` to the `ExploreAction` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "ExploreAction" ADD COLUMN     "stamina_cost" INTEGER NOT NULL;

-- AlterTable
ALTER TABLE "Hero" ADD COLUMN     "staminaRegenRate" INTEGER NOT NULL DEFAULT 1;

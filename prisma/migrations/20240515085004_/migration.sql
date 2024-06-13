/*
  Warnings:

  - You are about to drop the column `lifeStealEffectId` on the `MinionEffectEffect` table. All the data in the column will be lost.
  - You are about to drop the `LifeStealEffect` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "MinionEffectEffect" DROP CONSTRAINT "MinionEffectEffect_lifeStealEffectId_fkey";

-- AlterTable
ALTER TABLE "LifestealEffect" ADD COLUMN     "percentage" DOUBLE PRECISION NOT NULL DEFAULT 0;

-- AlterTable
ALTER TABLE "MinionEffectEffect" DROP COLUMN "lifeStealEffectId";

-- DropTable
DROP TABLE "LifeStealEffect";

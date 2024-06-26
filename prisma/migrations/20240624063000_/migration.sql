/*
  Warnings:

  - You are about to drop the column `type` on the `RelicEffect` table. All the data in the column will be lost.
  - Added the required column `effect_type` to the `RelicEffect` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "RelicEffect" DROP COLUMN "type",
ADD COLUMN     "effect_type" "EffectType" NOT NULL;

/*
  Warnings:

  - You are about to drop the column `efffectType` on the `CardEffect` table. All the data in the column will be lost.
  - Added the required column `effectType` to the `CardEffect` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "CardEffect" DROP COLUMN "efffectType",
ADD COLUMN     "effectType" "EffectType" NOT NULL;

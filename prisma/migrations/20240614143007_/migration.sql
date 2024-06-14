/*
  Warnings:

  - You are about to drop the column `type` on the `CardEffect` table. All the data in the column will be lost.
  - Added the required column `efffectType` to the `CardEffect` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "CardEffect" DROP COLUMN "type",
ADD COLUMN     "efffectType" "EffectType" NOT NULL;

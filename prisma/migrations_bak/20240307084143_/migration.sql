/*
  Warnings:

  - You are about to drop the column `value` on the `CardEffect` table. All the data in the column will be lost.

*/
-- DropIndex
DROP INDEX "CardEffect_cardId_effectId_key";

-- AlterTable
ALTER TABLE "CardEffect" DROP COLUMN "value";

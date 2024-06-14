/*
  Warnings:

  - You are about to drop the column `effectTypeId` on the `CardEffect` table. All the data in the column will be lost.
  - You are about to drop the `EffectType` table. If the table is not empty, all the data it contains will be lost.
  - Added the required column `name` to the `CardEffect` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "CardEffect" DROP CONSTRAINT "CardEffect_effectTypeId_fkey";

-- AlterTable
ALTER TABLE "CardEffect" DROP COLUMN "effectTypeId",
ADD COLUMN     "name" TEXT NOT NULL;

-- DropTable
DROP TABLE "EffectType";

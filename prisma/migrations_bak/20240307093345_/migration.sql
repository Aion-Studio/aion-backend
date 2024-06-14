/*
  Warnings:

  - You are about to drop the column `effectId` on the `CardEffect` table. All the data in the column will be lost.
  - You are about to drop the `Effect` table. If the table is not empty, all the data it contains will be lost.
  - A unique constraint covering the columns `[cardId,effectType]` on the table `CardEffect` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `effectType` to the `CardEffect` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "CardEffect" DROP CONSTRAINT "CardEffect_effectId_fkey";

-- DropIndex
DROP INDEX "CardEffect_cardId_effectId_key";

-- DropIndex
DROP INDEX "CardEffect_effectId_key";

-- AlterTable
ALTER TABLE "CardEffect" DROP COLUMN "effectId",
ADD COLUMN     "effectType" "EffectType" NOT NULL;

-- DropTable
DROP TABLE "Effect";

-- CreateIndex
CREATE UNIQUE INDEX "CardEffect_cardId_effectType_key" ON "CardEffect"("cardId", "effectType");

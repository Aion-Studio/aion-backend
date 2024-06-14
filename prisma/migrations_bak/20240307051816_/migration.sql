/*
  Warnings:

  - The primary key for the `CardEffect` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `duration` on the `CardEffect` table. All the data in the column will be lost.
  - You are about to drop the column `effect` on the `CardEffect` table. All the data in the column will be lost.
  - You are about to drop the column `id` on the `CardEffect` table. All the data in the column will be lost.
  - You are about to drop the column `value` on the `CardEffect` table. All the data in the column will be lost.
  - Added the required column `effectId` to the `CardEffect` table without a default value. This is not possible if the table is not empty.

*/
-- DropIndex
DROP INDEX "CardEffect_cardId_effect_key";

-- AlterTable
ALTER TABLE "CardEffect" DROP CONSTRAINT "CardEffect_pkey",
DROP COLUMN "duration",
DROP COLUMN "effect",
DROP COLUMN "id",
DROP COLUMN "value",
ADD COLUMN     "effectId" TEXT NOT NULL,
ADD CONSTRAINT "CardEffect_pkey" PRIMARY KEY ("cardId", "effectId");

-- CreateTable
CREATE TABLE "Effect" (
    "id" TEXT NOT NULL,
    "effect" "EffectType" NOT NULL,
    "value" INTEGER,
    "duration" INTEGER,

    CONSTRAINT "Effect_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_effectId_fkey" FOREIGN KEY ("effectId") REFERENCES "Effect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

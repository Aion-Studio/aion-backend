/*
  Warnings:

  - You are about to drop the column `duration` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `effectType` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `effectValue` on the `Card` table. All the data in the column will be lost.
  - Added the required column `damage` to the `Card` table without a default value. This is not possible if the table is not empty.
  - Added the required column `health` to the `Card` table without a default value. This is not possible if the table is not empty.
  - Added the required column `manaCost` to the `Card` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Card" DROP COLUMN "duration",
DROP COLUMN "effectType",
DROP COLUMN "effectValue",
ADD COLUMN     "damage" INTEGER NOT NULL,
ADD COLUMN     "health" INTEGER NOT NULL,
ADD COLUMN     "manaCost" INTEGER NOT NULL;

-- CreateTable
CREATE TABLE "CardEffect" (
    "id" TEXT NOT NULL,
    "duration" INTEGER NOT NULL DEFAULT 0,
    "effectType" "EffectEnum" NOT NULL DEFAULT 'PhysicalDamage',
    "cardId" TEXT NOT NULL,

    CONSTRAINT "CardEffect_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

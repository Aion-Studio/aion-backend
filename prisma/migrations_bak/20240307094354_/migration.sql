/*
  Warnings:

  - Added the required column `effectTypeId` to the `CardEffect` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "CardEffect" ADD COLUMN     "effectTypeId" TEXT NOT NULL;

-- DropEnum
DROP TYPE "EffectType";

-- CreateTable
CREATE TABLE "EffectType" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "EffectType_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "CardEffect" ADD CONSTRAINT "CardEffect_effectTypeId_fkey" FOREIGN KEY ("effectTypeId") REFERENCES "EffectType"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

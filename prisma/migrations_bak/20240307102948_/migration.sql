/*
  Warnings:

  - You are about to drop the column `name` on the `CardEffect` table. All the data in the column will be lost.
  - Added the required column `effectTypeId` to the `CardEffect` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "CardEffect" DROP COLUMN "name",
ADD COLUMN     "effectTypeId" TEXT NOT NULL;

-- CreateTable
CREATE TABLE "EffectType" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "EffectType_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "EffectType_name_key" ON "EffectType"("name");

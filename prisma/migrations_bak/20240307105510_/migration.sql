/*
  Warnings:

  - You are about to drop the column `damageType` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `damageValue` on the `Card` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Card" DROP COLUMN "damageType",
DROP COLUMN "damageValue",
ADD COLUMN     "effectValue" INTEGER;

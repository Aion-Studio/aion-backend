/*
  Warnings:

  - Added the required column `cardType` to the `Card` table without a default value. This is not possible if the table is not empty.

*/
-- CreateEnum
CREATE TYPE "CardType" AS ENUM ('Attack', 'Defensive', 'Utility');

-- AlterTable
ALTER TABLE "Card" ADD COLUMN     "cardType" "CardType" NOT NULL;

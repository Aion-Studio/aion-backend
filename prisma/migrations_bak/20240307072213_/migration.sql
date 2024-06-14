/*
  Warnings:

  - Added the required column `imgUrl` to the `Card` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Card" ADD COLUMN     "imgUrl" TEXT NOT NULL;

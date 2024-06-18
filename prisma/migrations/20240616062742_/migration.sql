/*
  Warnings:

  - Added the required column `regenRate` to the `Stamina` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Stamina" ADD COLUMN     "regenRate" INTEGER NOT NULL;

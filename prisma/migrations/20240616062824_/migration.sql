/*
  Warnings:

  - Added the required column `lastRegenTime` to the `Stamina` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Stamina" ADD COLUMN     "lastRegenTime" TIMESTAMP(3) NOT NULL;

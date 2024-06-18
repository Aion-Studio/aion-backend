/*
  Warnings:

  - Added the required column `level` to the `Hero` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Hero" ADD COLUMN     "level" INTEGER NOT NULL;

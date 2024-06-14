/*
  Warnings:

  - You are about to drop the column `location` on the `Leyline` table. All the data in the column will be lost.
  - Added the required column `name` to the `Leyline` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Leyline" RENAME COLUMN "location" TO "name";

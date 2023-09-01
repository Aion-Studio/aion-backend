/*
  Warnings:

  - Added the required column `aion_rate` to the `Leyline` table without a default value. This is not possible if the table is not empty.
  - Added the required column `discovery_required` to the `Leyline` table without a default value. This is not possible if the table is not empty.
  - Added the required column `stamina_rate` to the `Leyline` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Leyline" ADD COLUMN     "aion_rate" DOUBLE PRECISION NOT NULL,
ADD COLUMN     "discovery_required" INTEGER NOT NULL,
ADD COLUMN     "stamina_rate" DOUBLE PRECISION NOT NULL;

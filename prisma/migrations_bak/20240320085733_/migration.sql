/*
  Warnings:

  - You are about to drop the column `resilience` on the `Attributes` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Attributes" DROP COLUMN "resilience";

-- AlterTable
ALTER TABLE "BaseStats" ADD COLUMN     "resilience" INTEGER NOT NULL DEFAULT 0;

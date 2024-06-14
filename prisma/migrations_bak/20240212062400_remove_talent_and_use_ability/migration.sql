/*
  Warnings:

  - You are about to drop the column `heroId` on the `Talent` table. All the data in the column will be lost.

*/
-- DropForeignKey
ALTER TABLE "Talent" DROP CONSTRAINT "Talent_heroId_fkey";

-- AlterTable
ALTER TABLE "Talent" DROP COLUMN "heroId";

-- CreateTable
CREATE TABLE "Npc" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "level" INTEGER NOT NULL,
    "resilience" INTEGER NOT NULL,
    "strength" INTEGER NOT NULL,
    "agility" INTEGER NOT NULL,

    CONSTRAINT "Npc_pkey" PRIMARY KEY ("id")
);

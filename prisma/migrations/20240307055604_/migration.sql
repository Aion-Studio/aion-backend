/*
  Warnings:

  - The primary key for the `CardInDeck` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The required column `id` was added to the `CardInDeck` table with a prisma-level default value. This is not possible if the table is not empty. Please add this column as optional, then populate it before making it required.

*/
-- DropForeignKey
ALTER TABLE "CardInDeck" DROP CONSTRAINT "CardInDeck_heroCardId_fkey";

-- AlterTable
ALTER TABLE "CardInDeck" DROP CONSTRAINT "CardInDeck_pkey",
ADD COLUMN     "id" TEXT NOT NULL,
ADD CONSTRAINT "CardInDeck_pkey" PRIMARY KEY ("id");

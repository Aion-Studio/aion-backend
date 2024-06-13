/*
  Warnings:

  - You are about to drop the `Effect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `_CardEffects` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "_CardEffects" DROP CONSTRAINT "_CardEffects_A_fkey";

-- DropForeignKey
ALTER TABLE "_CardEffects" DROP CONSTRAINT "_CardEffects_B_fkey";

-- DropTable
DROP TABLE "Effect";

-- DropTable
DROP TABLE "_CardEffects";

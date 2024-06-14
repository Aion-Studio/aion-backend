/*
  Warnings:

  - The primary key for the `CardEffect` table will be changed. If it partially fails, the table could be left without primary key constraint.

*/
-- AlterTable
ALTER TABLE "CardEffect" DROP CONSTRAINT "CardEffect_pkey",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ADD CONSTRAINT "CardEffect_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "CardEffect_id_seq";

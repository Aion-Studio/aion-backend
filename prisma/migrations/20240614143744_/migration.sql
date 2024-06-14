/*
  Warnings:

  - You are about to drop the column `type` on the `Card` table. All the data in the column will be lost.
  - Added the required column `class` to the `Card` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Card" DROP COLUMN "type",
ADD COLUMN     "class" "Class" NOT NULL;

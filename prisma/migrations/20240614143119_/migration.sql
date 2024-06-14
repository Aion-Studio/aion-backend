/*
  Warnings:

  - Changed the type of `target` on the `CardEffect` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- AlterTable
ALTER TABLE "CardEffect" DROP COLUMN "target",
ADD COLUMN     "target" "TargetType" NOT NULL;

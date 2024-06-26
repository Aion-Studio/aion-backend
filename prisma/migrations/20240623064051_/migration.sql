/*
  Warnings:

  - A unique constraint covering the columns `[staminaId]` on the table `Hero` will be added. If there are existing duplicate values, this will fail.
  - Made the column `staminaId` on table `Hero` required. This step will fail if there are existing NULL values in that column.

*/
-- DropForeignKey
ALTER TABLE "Stamina" DROP CONSTRAINT "Stamina_heroId_fkey";

-- DropIndex
DROP INDEX "Stamina_heroId_key";

-- AlterTable
ALTER TABLE "Hero" ALTER COLUMN "staminaId" SET NOT NULL;

-- CreateIndex
CREATE UNIQUE INDEX "Hero_staminaId_key" ON "Hero"("staminaId");

-- AddForeignKey
ALTER TABLE "Hero" ADD CONSTRAINT "Hero_staminaId_fkey" FOREIGN KEY ("staminaId") REFERENCES "Stamina"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

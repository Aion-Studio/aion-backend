/*
  Warnings:

  - A unique constraint covering the columns `[heroId]` on the table `Stamina` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `heroId` to the `Stamina` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "Hero" DROP CONSTRAINT "Hero_staminaId_fkey";

-- DropIndex
DROP INDEX "Hero_staminaId_key";

-- AlterTable
ALTER TABLE "Stamina" ADD COLUMN     "heroId" TEXT NOT NULL;

-- CreateIndex
CREATE UNIQUE INDEX "Stamina_heroId_key" ON "Stamina"("heroId");

-- AddForeignKey
ALTER TABLE "Stamina" ADD CONSTRAINT "Stamina_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

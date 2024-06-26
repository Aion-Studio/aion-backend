/*
  Warnings:

  - You are about to drop the column `staminaId` on the `Hero` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[heroId]` on the table `Stamina` will be added. If there are existing duplicate values, this will fail.

*/
-- DropForeignKey
ALTER TABLE "Hero" DROP CONSTRAINT "Hero_staminaId_fkey";

-- DropIndex
DROP INDEX "Hero_staminaId_key";

-- AlterTable
ALTER TABLE "Hero" DROP COLUMN "staminaId";

-- CreateIndex
CREATE UNIQUE INDEX "Stamina_heroId_key" ON "Stamina"("heroId");

-- AddForeignKey
ALTER TABLE "Stamina" ADD CONSTRAINT "Stamina_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

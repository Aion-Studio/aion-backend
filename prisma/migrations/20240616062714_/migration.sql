/*
  Warnings:

  - You are about to drop the column `stamina` on the `Hero` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[staminaId]` on the table `Hero` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `staminaId` to the `Hero` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Hero" DROP COLUMN "stamina",
ADD COLUMN     "staminaId" TEXT NOT NULL;

-- CreateTable
CREATE TABLE "Stamina" (
    "id" TEXT NOT NULL,
    "capacity" INTEGER NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "Stamina_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Hero_staminaId_key" ON "Hero"("staminaId");

-- AddForeignKey
ALTER TABLE "Hero" ADD CONSTRAINT "Hero_staminaId_fkey" FOREIGN KEY ("staminaId") REFERENCES "Stamina"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

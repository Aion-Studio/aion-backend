/*
  Warnings:

  - A unique constraint covering the columns `[npcId]` on the table `Leyline` will be added. If there are existing duplicate values, this will fail.

*/
-- AlterTable
ALTER TABLE "Action" ADD COLUMN     "npc_id" TEXT;

-- AlterTable
ALTER TABLE "Leyline" ADD COLUMN     "npcId" TEXT NOT NULL DEFAULT 'a46877db-77cc-41de-b2a1-929303b467d0';

-- CreateIndex
CREATE UNIQUE INDEX "Leyline_npcId_key" ON "Leyline"("npcId");

-- AddForeignKey
ALTER TABLE "Action" ADD CONSTRAINT "Action_npc_id_fkey" FOREIGN KEY ("npc_id") REFERENCES "Npc"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Leyline" ADD CONSTRAINT "Leyline_npcId_fkey" FOREIGN KEY ("npcId") REFERENCES "Npc"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

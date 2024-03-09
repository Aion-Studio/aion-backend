/*
  Warnings:

  - A unique constraint covering the columns `[deckId]` on the table `Npc` will be added. If there are existing duplicate values, this will fail.

*/
-- AlterTable
ALTER TABLE "Npc" ADD COLUMN     "deckId" TEXT;

-- CreateIndex
CREATE UNIQUE INDEX "Npc_deckId_key" ON "Npc"("deckId");

-- AddForeignKey
ALTER TABLE "Npc" ADD CONSTRAINT "Npc_deckId_fkey" FOREIGN KEY ("deckId") REFERENCES "Deck"("id") ON DELETE SET NULL ON UPDATE CASCADE;

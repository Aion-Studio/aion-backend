/*
  Warnings:

  - You are about to drop the column `heroId` on the `Deck` table. All the data in the column will be lost.

*/
-- DropIndex
DROP INDEX "Deck_heroId_key";

-- AlterTable
ALTER TABLE "Deck" DROP COLUMN "heroId";

-- CreateTable
CREATE TABLE "_CardEffects" (
    "A" TEXT NOT NULL,
    "B" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_CardEffects_AB_unique" ON "_CardEffects"("A", "B");

-- CreateIndex
CREATE INDEX "_CardEffects_B_index" ON "_CardEffects"("B");

-- AddForeignKey
ALTER TABLE "_CardEffects" ADD CONSTRAINT "_CardEffects_A_fkey" FOREIGN KEY ("A") REFERENCES "Card"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_CardEffects" ADD CONSTRAINT "_CardEffects_B_fkey" FOREIGN KEY ("B") REFERENCES "Effect"("id") ON DELETE CASCADE ON UPDATE CASCADE;

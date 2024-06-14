/*
  Warnings:

  - The primary key for the `CardInDeck` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `id` on the `CardInDeck` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "CardInDeck" DROP CONSTRAINT "CardInDeck_pkey",
DROP COLUMN "id",
ADD CONSTRAINT "CardInDeck_pkey" PRIMARY KEY ("deckId", "heroCardId");

-- CreateTable
CREATE TABLE "Effect" (
    "id" TEXT NOT NULL,
    "effect" "EffectType" NOT NULL,
    "value" INTEGER,
    "duration" INTEGER,

    CONSTRAINT "Effect_pkey" PRIMARY KEY ("id")
);

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
ALTER TABLE "CardInDeck" ADD CONSTRAINT "CardInDeck_heroCardId_fkey" FOREIGN KEY ("heroCardId") REFERENCES "HeroCard"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_CardEffects" ADD CONSTRAINT "_CardEffects_A_fkey" FOREIGN KEY ("A") REFERENCES "Card"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_CardEffects" ADD CONSTRAINT "_CardEffects_B_fkey" FOREIGN KEY ("B") REFERENCES "Effect"("id") ON DELETE CASCADE ON UPDATE CASCADE;

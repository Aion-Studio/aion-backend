-- DropIndex
DROP INDEX "NpcCard_npcId_cardId_key";

-- AlterTable
ALTER TABLE "NpcCard" ADD COLUMN     "inDeck" BOOLEAN NOT NULL DEFAULT false;

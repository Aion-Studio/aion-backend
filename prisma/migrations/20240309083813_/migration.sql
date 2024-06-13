-- CreateTable
CREATE TABLE "NpcCard" (
    "id" TEXT NOT NULL,
    "npcId" TEXT NOT NULL,
    "cardId" TEXT NOT NULL,

    CONSTRAINT "NpcCard_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "NpcCard_npcId_cardId_key" ON "NpcCard"("npcId", "cardId");

-- AddForeignKey
ALTER TABLE "NpcCard" ADD CONSTRAINT "NpcCard_npcId_fkey" FOREIGN KEY ("npcId") REFERENCES "Npc"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "NpcCard" ADD CONSTRAINT "NpcCard_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

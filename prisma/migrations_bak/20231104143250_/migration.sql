-- CreateTable
CREATE TABLE "Action" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "region_name" TEXT NOT NULL,
    "leyline_id" TEXT,
    "quest_id" TEXT,

    CONSTRAINT "Action_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HeroActions" (
    "id" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "action_id" TEXT NOT NULL,

    CONSTRAINT "HeroActions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Quest" (
    "id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "region_name" TEXT NOT NULL,

    CONSTRAINT "Quest_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "HeroQuests" (
    "id" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "quest_id" TEXT NOT NULL,
    "completed" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "HeroQuests_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "_QuestRequirements" (
    "A" TEXT NOT NULL,
    "B" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_QuestRequirements_AB_unique" ON "_QuestRequirements"("A", "B");

-- CreateIndex
CREATE INDEX "_QuestRequirements_B_index" ON "_QuestRequirements"("B");

-- AddForeignKey
ALTER TABLE "Action" ADD CONSTRAINT "Action_region_name_fkey" FOREIGN KEY ("region_name") REFERENCES "Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Action" ADD CONSTRAINT "Action_leyline_id_fkey" FOREIGN KEY ("leyline_id") REFERENCES "Leyline"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Action" ADD CONSTRAINT "Action_quest_id_fkey" FOREIGN KEY ("quest_id") REFERENCES "Quest"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroActions" ADD CONSTRAINT "HeroActions_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroActions" ADD CONSTRAINT "HeroActions_action_id_fkey" FOREIGN KEY ("action_id") REFERENCES "Action"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Quest" ADD CONSTRAINT "Quest_region_name_fkey" FOREIGN KEY ("region_name") REFERENCES "Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroQuests" ADD CONSTRAINT "HeroQuests_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroQuests" ADD CONSTRAINT "HeroQuests_quest_id_fkey" FOREIGN KEY ("quest_id") REFERENCES "Quest"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_QuestRequirements" ADD CONSTRAINT "_QuestRequirements_A_fkey" FOREIGN KEY ("A") REFERENCES "Quest"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_QuestRequirements" ADD CONSTRAINT "_QuestRequirements_B_fkey" FOREIGN KEY ("B") REFERENCES "Quest"("id") ON DELETE CASCADE ON UPDATE CASCADE;

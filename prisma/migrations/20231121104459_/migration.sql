/*
  Warnings:

  - A unique constraint covering the columns `[name,description,quest_id]` on the table `Action` will be added. If there are existing duplicate values, this will fail.

*/
-- DropIndex
DROP INDEX "Action_name_region_name_quest_id_key";

-- CreateIndex
CREATE UNIQUE INDEX "Action_name_description_quest_id_key" ON "Action"("name", "description", "quest_id");

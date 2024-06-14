/*
  Warnings:

  - A unique constraint covering the columns `[hero_id,resource_type_id]` on the table `HeroResource` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX "HeroResource_hero_id_resource_type_id_key" ON "HeroResource"("hero_id", "resource_type_id");

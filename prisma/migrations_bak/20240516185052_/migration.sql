/*
  Warnings:

  - A unique constraint covering the columns `[name,nation]` on the table `Card` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX "Card_name_nation_key" ON "Card"("name", "nation");

/*
  Warnings:

  - A unique constraint covering the columns `[type]` on the table `Common` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX "Common_type_key" ON "Common"("type");

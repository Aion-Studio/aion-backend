/*
  Warnings:

  - A unique constraint covering the columns `[type,id]` on the table `ResourceType` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX "ResourceType_type_id_key" ON "ResourceType"("type", "id");

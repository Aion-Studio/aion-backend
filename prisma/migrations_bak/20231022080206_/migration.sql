/*
  Warnings:

  - A unique constraint covering the columns `[type]` on the table `ResourceType` will be added. If there are existing duplicate values, this will fail.

*/
-- DropIndex
DROP INDEX "ResourceType_type_id_key";

-- CreateIndex
CREATE UNIQUE INDEX "ResourceType_type_key" ON "ResourceType"("type");

/*
  Warnings:

  - You are about to drop the column `resourceTypeId` on the `HeroResource` table. All the data in the column will be lost.
  - Added the required column `resource_type_id` to the `HeroResource` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "HeroResource" DROP CONSTRAINT "HeroResource_resourceTypeId_fkey";

-- AlterTable
ALTER TABLE "HeroResource" DROP COLUMN "resourceTypeId",
ADD COLUMN     "resource_type_id" TEXT NOT NULL;

-- AddForeignKey
ALTER TABLE "HeroResource" ADD CONSTRAINT "HeroResource_resource_type_id_fkey" FOREIGN KEY ("resource_type_id") REFERENCES "ResourceType"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

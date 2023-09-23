/*
  Warnings:

  - You are about to drop the `ActionResult` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `ActionResultResource` table. If the table is not empty, all the data it contains will be lost.

*/
-- AlterEnum
ALTER TYPE "ResourceType" ADD VALUE 'Stamina';

-- DropForeignKey
ALTER TABLE "ActionResult" DROP CONSTRAINT "ActionResult_hero_id_fkey";

-- DropForeignKey
ALTER TABLE "ActionResultResource" DROP CONSTRAINT "ActionResultResource_action_result_id_fkey";

-- DropTable
DROP TABLE "ActionResult";

-- DropTable
DROP TABLE "ActionResultResource";

-- CreateTable
CREATE TABLE "ActionCompleted" (
    "id" TEXT NOT NULL,
    "action_name" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "create_time" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "ActionCompleted_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "ActionCompleted" ADD CONSTRAINT "ActionCompleted_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

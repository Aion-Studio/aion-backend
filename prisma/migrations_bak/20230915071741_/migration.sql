/*
  Warnings:

  - You are about to drop the `RegionActionResult` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `ResourceValue` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "RegionActionResult" DROP CONSTRAINT "RegionActionResult_hero_id_fkey";

-- DropForeignKey
ALTER TABLE "ResourceValue" DROP CONSTRAINT "ResourceValue_regionActionResultId_fkey";

-- DropTable
DROP TABLE "RegionActionResult";

-- DropTable
DROP TABLE "ResourceValue";

-- CreateTable
CREATE TABLE "ActionResult" (
    "id" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "xp" INTEGER NOT NULL,
    "discovery_level_increase" DOUBLE PRECISION NOT NULL,

    CONSTRAINT "ActionResult_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "ActionResultResource" (
    "id" TEXT NOT NULL,
    "type" "ResourceType" NOT NULL,
    "amount" INTEGER NOT NULL,
    "action_result_id" TEXT NOT NULL,

    CONSTRAINT "ActionResultResource_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "ActionResult" ADD CONSTRAINT "ActionResult_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "ActionResultResource" ADD CONSTRAINT "ActionResultResource_action_result_id_fkey" FOREIGN KEY ("action_result_id") REFERENCES "ActionResult"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

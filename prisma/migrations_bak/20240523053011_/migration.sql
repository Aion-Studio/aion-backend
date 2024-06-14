/*
  Warnings:

  - You are about to drop the `DeckCard` table. If the table is not empty, all the data it contains will be lost.
  - A unique constraint covering the columns `[instanceId]` on the table `HeroCard` will be added. If there are existing duplicate values, this will fail.
  - The required column `instanceId` was added to the `HeroCard` table with a prisma-level default value. This is not possible if the table is not empty. Please add this column as optional, then populate it before making it required.

*/
/* use uuid ext */
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

ALTER TABLE "DeckCard" DROP CONSTRAINT "DeckCard_cardId_fkey";
-- migration.sql

-- 1. Add instanceId column (optional)
ALTER TABLE "HeroCard"
ADD COLUMN "instanceId" VARCHAR(255);

-- 2. Populate instanceId for existing rows
UPDATE "HeroCard"
SET "instanceId" =  uuid_generate_v4()
WHERE "instanceId" IS NULL;

-- 3. Make instanceId required
ALTER TABLE "HeroCard"
ALTER COLUMN "instanceId" SET NOT NULL;

-- 4. Add inDeck column (default to false)
ALTER TABLE "HeroCard"
ADD COLUMN "inDeck" BOOLEAN DEFAULT FALSE;


-- DropTable
DROP TABLE "DeckCard";

-- CreateIndex
CREATE UNIQUE INDEX "HeroCard_instanceId_key" ON "HeroCard"("instanceId");

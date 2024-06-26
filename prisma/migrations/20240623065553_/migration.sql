-- DropForeignKey
ALTER TABLE "Stamina" DROP CONSTRAINT "Stamina_heroId_fkey";

-- AlterTable
ALTER TABLE "Stamina" ALTER COLUMN "heroId" DROP NOT NULL;

-- AddForeignKey
ALTER TABLE "Stamina" ADD CONSTRAINT "Stamina_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE SET NULL ON UPDATE CASCADE;

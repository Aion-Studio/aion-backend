-- AlterTable
ALTER TABLE "MinionEffectEffect" ADD COLUMN     "poisonEffectId" TEXT;

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_poisonEffectId_fkey" FOREIGN KEY ("poisonEffectId") REFERENCES "PoisonEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_stunEffectId_fkey";

-- AlterTable
ALTER TABLE "SpellEffect" ALTER COLUMN "stunEffectId" DROP NOT NULL;

-- AddForeignKey
ALTER TABLE "SpellEffect" ADD CONSTRAINT "SpellEffect_stunEffectId_fkey" FOREIGN KEY ("stunEffectId") REFERENCES "StunEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

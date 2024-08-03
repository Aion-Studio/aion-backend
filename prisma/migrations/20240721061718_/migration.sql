/*
  Warnings:

  - You are about to drop the column `effect` on the `SpellEffect` table. All the data in the column will be lost.
  - Added the required column `effectType` to the `SpellEffect` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Spell" ADD COLUMN     "npcId" TEXT;

-- AlterTable
ALTER TABLE "SpellEffect" DROP COLUMN "effect",
ADD COLUMN     "damageType" "DamageType" DEFAULT 'Normal',
ADD COLUMN     "duration" INTEGER,
ADD COLUMN     "effectType" "EffectType" NOT NULL,
ADD COLUMN     "percentageModifier" BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN     "statAffected" "StatType";

-- AddForeignKey
ALTER TABLE "Spell" ADD CONSTRAINT "Spell_npcId_fkey" FOREIGN KEY ("npcId") REFERENCES "Npc"("id") ON DELETE SET NULL ON UPDATE CASCADE;

/*
  Warnings:

  - You are about to drop the column `damageEffectId` on the `SpellEffectEffect` table. All the data in the column will be lost.

*/
-- DropForeignKey
ALTER TABLE "SpellEffectEffect" DROP CONSTRAINT "SpellEffectEffect_damageEffectId_fkey";

-- AlterTable
ALTER TABLE "SpellEffectEffect" DROP COLUMN "damageEffectId";

-- CreateTable
CREATE TABLE "_DamageEffectToSpellEffectEffect" (
    "A" TEXT NOT NULL,
    "B" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_DamageEffectToSpellEffectEffect_AB_unique" ON "_DamageEffectToSpellEffectEffect"("A", "B");

-- CreateIndex
CREATE INDEX "_DamageEffectToSpellEffectEffect_B_index" ON "_DamageEffectToSpellEffectEffect"("B");

-- AddForeignKey
ALTER TABLE "_DamageEffectToSpellEffectEffect" ADD CONSTRAINT "_DamageEffectToSpellEffectEffect_A_fkey" FOREIGN KEY ("A") REFERENCES "DamageEffect"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_DamageEffectToSpellEffectEffect" ADD CONSTRAINT "_DamageEffectToSpellEffectEffect_B_fkey" FOREIGN KEY ("B") REFERENCES "SpellEffectEffect"("id") ON DELETE CASCADE ON UPDATE CASCADE;

/*
  Warnings:

  - You are about to drop the column `amount` on the `DamageEffect` table. All the data in the column will be lost.
  - You are about to drop the column `damageType` on the `DamageEffect` table. All the data in the column will be lost.
  - You are about to drop the column `targetType` on the `DamageEffect` table. All the data in the column will be lost.
  - You are about to drop the `_DamageEffectToSpellEffectEffect` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "_DamageEffectToSpellEffectEffect" DROP CONSTRAINT "_DamageEffectToSpellEffectEffect_A_fkey";

-- DropForeignKey
ALTER TABLE "_DamageEffectToSpellEffectEffect" DROP CONSTRAINT "_DamageEffectToSpellEffectEffect_B_fkey";

-- AlterTable
ALTER TABLE "DamageEffect" DROP COLUMN "amount",
DROP COLUMN "damageType",
DROP COLUMN "targetType";

-- AlterTable
ALTER TABLE "SpellEffectEffect" ADD COLUMN     "damageEffectId" TEXT;

-- DropTable
DROP TABLE "_DamageEffectToSpellEffectEffect";

-- CreateTable
CREATE TABLE "DamageEffectDetails" (
    "id" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,
    "damageType" "DamageType" NOT NULL,
    "targetType" "TargetType" NOT NULL,
    "damageEffectId" TEXT,

    CONSTRAINT "DamageEffectDetails_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_damageEffectId_fkey" FOREIGN KEY ("damageEffectId") REFERENCES "DamageEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DamageEffectDetails" ADD CONSTRAINT "DamageEffectDetails_damageEffectId_fkey" FOREIGN KEY ("damageEffectId") REFERENCES "DamageEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

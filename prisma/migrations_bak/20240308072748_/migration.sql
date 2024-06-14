/*
  Warnings:

  - You are about to drop the `ArmorEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `DamageEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `HealEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `InitiativeEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `PoisonEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `ResilienceEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `StunEffect` table. If the table is not empty, all the data it contains will be lost.

*/
-- AlterTable
ALTER TABLE "SpellEffect" ADD COLUMN     "armorAmount" INTEGER,
ADD COLUMN     "damageAmount" INTEGER,
ADD COLUMN     "damageType" "DamageType",
ADD COLUMN     "healAmount" INTEGER,
ADD COLUMN     "initiativeAmount" INTEGER,
ADD COLUMN     "poisonAmount" INTEGER,
ADD COLUMN     "resilienceAmount" INTEGER,
ADD COLUMN     "stunDuration" INTEGER,
ADD COLUMN     "targetType" "TargetType";

-- DropTable
DROP TABLE "ArmorEffect";

-- DropTable
DROP TABLE "DamageEffect";

-- DropTable
DROP TABLE "HealEffect";

-- DropTable
DROP TABLE "InitiativeEffect";

-- DropTable
DROP TABLE "PoisonEffect";

-- DropTable
DROP TABLE "ResilienceEffect";

-- DropTable
DROP TABLE "StunEffect";

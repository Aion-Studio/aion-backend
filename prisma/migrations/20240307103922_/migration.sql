/*
  Warnings:

  - Changed the type of `effectTypeId` on the `CardEffect` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- CreateEnum
CREATE TYPE "EffectEnum" AS ENUM ('PhysicalDamage', 'SpellDamage', 'ChaosDamage', 'DamageOverTime', 'Stun', 'ReduceArmor', 'ReduceResilience', 'IncreaseArmor', 'IncreaseResilience', 'Heal', 'HealOverTime', 'DrawCards', 'ApplyPoison', 'RemovePoison', 'ApplyInitiative', 'RemoveInitiative');

-- AlterTable
ALTER TABLE "CardEffect" DROP COLUMN "effectTypeId",
ADD COLUMN     "effectTypeId" "EffectEnum" NOT NULL;

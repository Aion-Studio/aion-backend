/*
  Warnings:

  - The values [Buff,Debuff] on the enum `EffectType` will be removed. If these variants are still used in the database, this will fail.

*/
-- CreateEnum
CREATE TYPE "StatType" AS ENUM ('HP', 'Strength', 'Dexterity', 'Intelligence', 'Armor');

-- AlterEnum
BEGIN;
CREATE TYPE "EffectType_new" AS ENUM ('Damage', 'Heal', 'BuffStat', 'BuffDamage', 'DebuffStat', 'DebuffDamage', 'ManaGain', 'Draw', 'Armor', 'Silence', 'Poison', 'Initiative');
ALTER TABLE "RelicEffect" ALTER COLUMN "effect_type" TYPE "EffectType_new" USING ("effect_type"::text::"EffectType_new");
ALTER TABLE "SpellEffect" ALTER COLUMN "effect" TYPE "EffectType_new" USING ("effect"::text::"EffectType_new");
ALTER TABLE "CardEffect" ALTER COLUMN "efffectType" TYPE "EffectType_new" USING ("efffectType"::text::"EffectType_new");
ALTER TYPE "EffectType" RENAME TO "EffectType_old";
ALTER TYPE "EffectType_new" RENAME TO "EffectType";
DROP TYPE "EffectType_old";
COMMIT;

-- AlterTable
ALTER TABLE "CardEffect" ADD COLUMN     "percentageModifier" BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN     "statAffected" "StatType";

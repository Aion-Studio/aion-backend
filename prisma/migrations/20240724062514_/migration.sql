/*
  Warnings:

  - The values [BuffStat,DebuffStat,Armor,Initiative] on the enum `EffectType` will be removed. If these variants are still used in the database, this will fail.

*/
-- AlterEnum
BEGIN;
CREATE TYPE "EffectType_new" AS ENUM ('Damage', 'Heal', 'BuffDamage', 'DebuffDamage', 'ManaGain', 'Draw', 'BuffArmor', 'DebuffArmor', 'Silence', 'Poison', 'BuffInitiative', 'DebuffInitiative');
ALTER TABLE "RelicEffect" ALTER COLUMN "effect_type" TYPE "EffectType_new" USING ("effect_type"::text::"EffectType_new");
ALTER TABLE "SpellEffect" ALTER COLUMN "effectType" TYPE "EffectType_new" USING ("effectType"::text::"EffectType_new");
ALTER TABLE "CardEffect" ALTER COLUMN "effectType" TYPE "EffectType_new" USING ("effectType"::text::"EffectType_new");
ALTER TYPE "EffectType" RENAME TO "EffectType_old";
ALTER TYPE "EffectType_new" RENAME TO "EffectType";
DROP TYPE "EffectType_old";
COMMIT;

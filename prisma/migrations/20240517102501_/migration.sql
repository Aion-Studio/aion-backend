-- AlterTable
ALTER TABLE "DamageEffectDetails" ALTER COLUMN "damageType" SET DEFAULT 'Physical',
ALTER COLUMN "targetType" DROP NOT NULL;

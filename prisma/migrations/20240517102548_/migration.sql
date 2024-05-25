/*
  Warnings:

  - Made the column `targetType` on table `DamageEffectDetails` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "DamageEffectDetails" ALTER COLUMN "targetType" SET NOT NULL,
ALTER COLUMN "targetType" SET DEFAULT 'Both';

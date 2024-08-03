-- CreateEnum
CREATE TYPE "DamageType" AS ENUM ('Normal', 'Chaos');

-- AlterTable
ALTER TABLE "CardEffect" ADD COLUMN     "damageType" "DamageType" DEFAULT 'Normal';

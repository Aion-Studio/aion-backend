-- AlterTable
ALTER TABLE "MinionEffectEffect" ADD COLUMN     "dyingWishDamageEffectId" TEXT;

-- CreateTable
CREATE TABLE "DyingWishDamageEffect" (
    "id" TEXT NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "DyingWishDamageEffect_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "MinionEffectEffect" ADD CONSTRAINT "MinionEffectEffect_dyingWishDamageEffectId_fkey" FOREIGN KEY ("dyingWishDamageEffectId") REFERENCES "DyingWishDamageEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AlterTable
ALTER TABLE "SpellEffectEffect" ADD COLUMN     "dazeEffectId" TEXT;

-- CreateTable
CREATE TABLE "DazeEffect" (
    "id" TEXT NOT NULL,

    CONSTRAINT "DazeEffect_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "SpellEffectEffect" ADD CONSTRAINT "SpellEffectEffect_dazeEffectId_fkey" FOREIGN KEY ("dazeEffectId") REFERENCES "DazeEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

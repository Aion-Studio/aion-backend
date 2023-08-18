-- CreateTable
CREATE TABLE "HeroResource" (
    "id" TEXT NOT NULL,
    "hero_id" TEXT NOT NULL,
    "resource" "ResourceType" NOT NULL,
    "amount" INTEGER NOT NULL,

    CONSTRAINT "HeroResource_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "HeroResource" ADD CONSTRAINT "HeroResource_hero_id_fkey" FOREIGN KEY ("hero_id") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- CreateTable
CREATE TABLE "Talent" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT,
    "cooldown" INTEGER NOT NULL,
    "heroId" TEXT,
    "effects" JSONB NOT NULL,

    CONSTRAINT "Talent_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "FollowerAbility" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT,
    "cooldown" INTEGER NOT NULL,
    "effects" JSONB NOT NULL,
    "followerId" TEXT,

    CONSTRAINT "FollowerAbility_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Ability" (
    "id" TEXT NOT NULL,
    "heroId" TEXT,
    "talentId" TEXT,
    "followerAbilityId" TEXT,

    CONSTRAINT "Ability_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "Talent" ADD CONSTRAINT "Talent_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "FollowerAbility" ADD CONSTRAINT "FollowerAbility_followerId_fkey" FOREIGN KEY ("followerId") REFERENCES "Follower"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Ability" ADD CONSTRAINT "Ability_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Ability" ADD CONSTRAINT "Ability_talentId_fkey" FOREIGN KEY ("talentId") REFERENCES "Talent"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Ability" ADD CONSTRAINT "Ability_followerAbilityId_fkey" FOREIGN KEY ("followerAbilityId") REFERENCES "FollowerAbility"("id") ON DELETE SET NULL ON UPDATE CASCADE;

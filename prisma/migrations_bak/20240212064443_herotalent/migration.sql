/*
  Warnings:

  - You are about to drop the `Ability` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `FollowerAbility` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "Ability" DROP CONSTRAINT "Ability_followerAbilityId_fkey";

-- DropForeignKey
ALTER TABLE "Ability" DROP CONSTRAINT "Ability_heroId_fkey";

-- DropForeignKey
ALTER TABLE "Ability" DROP CONSTRAINT "Ability_talentId_fkey";

-- DropForeignKey
ALTER TABLE "FollowerAbility" DROP CONSTRAINT "FollowerAbility_followerId_fkey";

-- DropTable
DROP TABLE "Ability";

-- DropTable
DROP TABLE "FollowerAbility";

-- CreateTable
CREATE TABLE "HeroTalent" (
    "heroId" TEXT NOT NULL,
    "talentId" TEXT NOT NULL,

    CONSTRAINT "HeroTalent_pkey" PRIMARY KEY ("heroId","talentId")
);

-- CreateTable
CREATE TABLE "FollowerTalent" (
    "followerId" TEXT NOT NULL,
    "talentId" TEXT NOT NULL,

    CONSTRAINT "FollowerTalent_pkey" PRIMARY KEY ("followerId","talentId")
);

-- AddForeignKey
ALTER TABLE "HeroTalent" ADD CONSTRAINT "HeroTalent_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "HeroTalent" ADD CONSTRAINT "HeroTalent_talentId_fkey" FOREIGN KEY ("talentId") REFERENCES "Talent"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "FollowerTalent" ADD CONSTRAINT "FollowerTalent_followerId_fkey" FOREIGN KEY ("followerId") REFERENCES "Follower"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "FollowerTalent" ADD CONSTRAINT "FollowerTalent_talentId_fkey" FOREIGN KEY ("talentId") REFERENCES "Talent"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

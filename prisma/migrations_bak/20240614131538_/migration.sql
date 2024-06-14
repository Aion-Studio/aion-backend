/*
  Warnings:

  - The values [Minion,Hero,Both] on the enum `TargetType` will be removed. If these variants are still used in the database, this will fail.
  - You are about to drop the column `cardType` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `damage` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `deckLimit` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `health` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `imgUrl` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `manaCost` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `nation` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `rarity` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `account_id` on the `Hero` table. All the data in the column will be lost.
  - You are about to drop the column `aionCapacity` on the `Hero` table. All the data in the column will be lost.
  - You are about to drop the column `attributes_id` on the `Hero` table. All the data in the column will be lost.
  - You are about to drop the column `base_stats_id` on the `Hero` table. All the data in the column will be lost.
  - You are about to drop the column `inventory_id` on the `Hero` table. All the data in the column will be lost.
  - You are about to drop the column `last_stamina_regeneration_time` on the `Hero` table. All the data in the column will be lost.
  - You are about to drop the column `name` on the `Hero` table. All the data in the column will be lost.
  - You are about to drop the column `staminaMax` on the `Hero` table. All the data in the column will be lost.
  - You are about to drop the column `staminaRegenRate` on the `Hero` table. All the data in the column will be lost.
  - You are about to drop the column `deckCardId` on the `HeroCard` table. All the data in the column will be lost.
  - You are about to drop the column `instanceId` on the `HeroCard` table. All the data in the column will be lost.
  - You are about to drop the column `agility` on the `Npc` table. All the data in the column will be lost.
  - You are about to drop the column `deckId` on the `Npc` table. All the data in the column will be lost.
  - You are about to drop the column `resilience` on the `Npc` table. All the data in the column will be lost.
  - You are about to drop the `ArmorEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `Attributes` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `BaseStats` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `BattleCryEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `BlockEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `CardEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `ChargeEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `CleanseEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `CowardiceCurseEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `DamageEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `DamageEffectDetails` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `DazeEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `DyingWishDamageEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `DyingWishHealEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `EtherealEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `Follower` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `FollowerTalent` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `HealEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `HeroTalent` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `InitiativeEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `Inventory` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `Item` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `LifestealEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `MinionEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `NpcCard` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `PhantomTouchEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `PickupEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `PoisonEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `ResilienceEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `RetinueSlot` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `RoarAuraEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `SpellEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `SprayOfKnivesEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `StunEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `Talent` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `TauntEffect` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `TwinEffect` table. If the table is not empty, all the data it contains will be lost.
  - Added the required column `cost` to the `Card` table without a default value. This is not possible if the table is not empty.
  - Added the required column `effects` to the `Card` table without a default value. This is not possible if the table is not empty.
  - Added the required column `type` to the `Card` table without a default value. This is not possible if the table is not empty.
  - Added the required column `class` to the `Hero` table without a default value. This is not possible if the table is not empty.
  - Added the required column `crafting` to the `Hero` table without a default value. This is not possible if the table is not empty.
  - Added the required column `dexterity` to the `Hero` table without a default value. This is not possible if the table is not empty.
  - Added the required column `explore` to the `Hero` table without a default value. This is not possible if the table is not empty.
  - Added the required column `hitpoints` to the `Hero` table without a default value. This is not possible if the table is not empty.
  - Added the required column `intelligence` to the `Hero` table without a default value. This is not possible if the table is not empty.
  - Added the required column `strength` to the `Hero` table without a default value. This is not possible if the table is not empty.
  - Changed the type of `resource` on the `ResourceCost` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `type` on the `ResourceType` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- CreateEnum
CREATE TYPE "Class" AS ENUM ('Fighter', 'Ranger', 'Wizard');

-- CreateEnum
CREATE TYPE "EffectType" AS ENUM ('Heal', 'Damage', 'Buff', 'Debuff', 'ApplySpell');

-- CreateEnum
CREATE TYPE "Resource" AS ENUM ('Aion', 'Flux', 'Gem');

-- AlterEnum
BEGIN;
CREATE TYPE "TargetType_new" AS ENUM ('Opponent', 'Self');
ALTER TABLE "DamageEffectDetails" ALTER COLUMN "targetType" DROP DEFAULT;
ALTER TYPE "TargetType" RENAME TO "TargetType_old";
ALTER TYPE "TargetType_new" RENAME TO "TargetType";
DROP TYPE "TargetType_old";
COMMIT;

-- DropForeignKey
ALTER TABLE "CardEffect" DROP CONSTRAINT "CardEffect_minionEffectId_fkey";

-- DropForeignKey
ALTER TABLE "CardEffect" DROP CONSTRAINT "CardEffect_spellEffectId_fkey";

-- DropForeignKey
ALTER TABLE "DamageEffectDetails" DROP CONSTRAINT "DamageEffectDetails_damageEffectId_fkey";

-- DropForeignKey
ALTER TABLE "Follower" DROP CONSTRAINT "Follower_attributes_id_fkey";

-- DropForeignKey
ALTER TABLE "FollowerTalent" DROP CONSTRAINT "FollowerTalent_followerId_fkey";

-- DropForeignKey
ALTER TABLE "FollowerTalent" DROP CONSTRAINT "FollowerTalent_talentId_fkey";

-- DropForeignKey
ALTER TABLE "Hero" DROP CONSTRAINT "Hero_account_id_fkey";

-- DropForeignKey
ALTER TABLE "Hero" DROP CONSTRAINT "Hero_attributes_id_fkey";

-- DropForeignKey
ALTER TABLE "Hero" DROP CONSTRAINT "Hero_base_stats_id_fkey";

-- DropForeignKey
ALTER TABLE "Hero" DROP CONSTRAINT "Hero_inventory_id_fkey";

-- DropForeignKey
ALTER TABLE "HeroCard" DROP CONSTRAINT "HeroCard_deckCardId_fkey";

-- DropForeignKey
ALTER TABLE "HeroTalent" DROP CONSTRAINT "HeroTalent_heroId_fkey";

-- DropForeignKey
ALTER TABLE "HeroTalent" DROP CONSTRAINT "HeroTalent_talentId_fkey";

-- DropForeignKey
ALTER TABLE "Item" DROP CONSTRAINT "Item_active_inventory_id_fkey";

-- DropForeignKey
ALTER TABLE "Item" DROP CONSTRAINT "Item_backpack_inventory_id_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_blockEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_chargeEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_cleanseEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_dyingWishDamageEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_dyingWishHealEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_etherealEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_lifestealEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_pickupEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_roarAuraEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_tauntEffectId_fkey";

-- DropForeignKey
ALTER TABLE "MinionEffect" DROP CONSTRAINT "MinionEffect_twinEffectId_fkey";

-- DropForeignKey
ALTER TABLE "Npc" DROP CONSTRAINT "Npc_deckId_fkey";

-- DropForeignKey
ALTER TABLE "NpcCard" DROP CONSTRAINT "NpcCard_cardId_fkey";

-- DropForeignKey
ALTER TABLE "NpcCard" DROP CONSTRAINT "NpcCard_npcId_fkey";

-- DropForeignKey
ALTER TABLE "RetinueSlot" DROP CONSTRAINT "RetinueSlot_followerId_fkey";

-- DropForeignKey
ALTER TABLE "RetinueSlot" DROP CONSTRAINT "RetinueSlot_hero_id_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_armorEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_battleCryEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_cowardiceCurseEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_damageEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_dazeEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_healEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_initiativeEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_phantomTouchEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_poisonEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_resilienceEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_sprayOfKnivesEffectId_fkey";

-- DropForeignKey
ALTER TABLE "SpellEffect" DROP CONSTRAINT "SpellEffect_stunEffectId_fkey";

-- DropIndex
DROP INDEX "Card_name_nation_key";

-- DropIndex
DROP INDEX "Hero_attributes_id_key";

-- DropIndex
DROP INDEX "Hero_base_stats_id_key";

-- DropIndex
DROP INDEX "Hero_inventory_id_key";

-- DropIndex
DROP INDEX "HeroCard_deckCardId_key";

-- DropIndex
DROP INDEX "HeroCard_instanceId_key";

-- DropIndex
DROP INDEX "Npc_deckId_key";

-- AlterTable
ALTER TABLE "Card" DROP COLUMN "cardType",
DROP COLUMN "damage",
DROP COLUMN "deckLimit",
DROP COLUMN "health",
DROP COLUMN "imgUrl",
DROP COLUMN "manaCost",
DROP COLUMN "nation",
DROP COLUMN "rarity",
ADD COLUMN     "cost" INTEGER NOT NULL,
ADD COLUMN     "effects" TEXT NOT NULL,
ADD COLUMN     "type" TEXT NOT NULL;

-- AlterTable
ALTER TABLE "Deck" ADD COLUMN     "npcId" TEXT;

-- AlterTable
ALTER TABLE "DeckCard" ADD COLUMN     "heroCardId" TEXT;

-- AlterTable
ALTER TABLE "Hero" DROP COLUMN "account_id",
DROP COLUMN "aionCapacity",
DROP COLUMN "attributes_id",
DROP COLUMN "base_stats_id",
DROP COLUMN "inventory_id",
DROP COLUMN "last_stamina_regeneration_time",
DROP COLUMN "name",
DROP COLUMN "staminaMax",
DROP COLUMN "staminaRegenRate",
ADD COLUMN     "accountId" TEXT,
ADD COLUMN     "class" "Class" NOT NULL,
ADD COLUMN     "crafting" INTEGER NOT NULL,
ADD COLUMN     "dexterity" INTEGER NOT NULL,
ADD COLUMN     "explore" INTEGER NOT NULL,
ADD COLUMN     "hitpoints" INTEGER NOT NULL,
ADD COLUMN     "intelligence" INTEGER NOT NULL,
ADD COLUMN     "strength" INTEGER NOT NULL,
ALTER COLUMN "stamina" DROP DEFAULT;

-- AlterTable
ALTER TABLE "HeroCard" DROP COLUMN "deckCardId",
DROP COLUMN "instanceId";

-- AlterTable
ALTER TABLE "Npc" DROP COLUMN "agility",
DROP COLUMN "deckId",
DROP COLUMN "resilience";

-- AlterTable
ALTER TABLE "ResourceCost" DROP COLUMN "resource",
ADD COLUMN     "resource" "Resource" NOT NULL;

-- AlterTable
ALTER TABLE "ResourceType" DROP COLUMN "type",
ADD COLUMN     "type" "Resource" NOT NULL;

-- DropTable
DROP TABLE "ArmorEffect";

-- DropTable
DROP TABLE "Attributes";

-- DropTable
DROP TABLE "BaseStats";

-- DropTable
DROP TABLE "BattleCryEffect";

-- DropTable
DROP TABLE "BlockEffect";

-- DropTable
DROP TABLE "CardEffect";

-- DropTable
DROP TABLE "ChargeEffect";

-- DropTable
DROP TABLE "CleanseEffect";

-- DropTable
DROP TABLE "CowardiceCurseEffect";

-- DropTable
DROP TABLE "DamageEffect";

-- DropTable
DROP TABLE "DamageEffectDetails";

-- DropTable
DROP TABLE "DazeEffect";

-- DropTable
DROP TABLE "DyingWishDamageEffect";

-- DropTable
DROP TABLE "DyingWishHealEffect";

-- DropTable
DROP TABLE "EtherealEffect";

-- DropTable
DROP TABLE "Follower";

-- DropTable
DROP TABLE "FollowerTalent";

-- DropTable
DROP TABLE "HealEffect";

-- DropTable
DROP TABLE "HeroTalent";

-- DropTable
DROP TABLE "InitiativeEffect";

-- DropTable
DROP TABLE "Inventory";

-- DropTable
DROP TABLE "Item";

-- DropTable
DROP TABLE "LifestealEffect";

-- DropTable
DROP TABLE "MinionEffect";

-- DropTable
DROP TABLE "NpcCard";

-- DropTable
DROP TABLE "PhantomTouchEffect";

-- DropTable
DROP TABLE "PickupEffect";

-- DropTable
DROP TABLE "PoisonEffect";

-- DropTable
DROP TABLE "ResilienceEffect";

-- DropTable
DROP TABLE "RetinueSlot";

-- DropTable
DROP TABLE "RoarAuraEffect";

-- DropTable
DROP TABLE "SpellEffect";

-- DropTable
DROP TABLE "SprayOfKnivesEffect";

-- DropTable
DROP TABLE "StunEffect";

-- DropTable
DROP TABLE "Talent";

-- DropTable
DROP TABLE "TauntEffect";

-- DropTable
DROP TABLE "TwinEffect";

-- DropEnum
DROP TYPE "CardType";

-- DropEnum
DROP TYPE "DamageType";

-- DropEnum
DROP TYPE "ResourceEnum";

-- CreateTable
CREATE TABLE "Relic" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "heroId" TEXT NOT NULL,

    CONSTRAINT "Relic_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "RelicEffect" (
    "id" TEXT NOT NULL,
    "type" "EffectType" NOT NULL,
    "value" INTEGER NOT NULL,
    "target" TEXT NOT NULL,
    "duration" INTEGER,
    "relicId" TEXT NOT NULL,

    CONSTRAINT "RelicEffect_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Spell" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "slot" INTEGER NOT NULL,
    "level" INTEGER NOT NULL,
    "heroId" TEXT NOT NULL,

    CONSTRAINT "Spell_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "ResourceType_type_key" ON "ResourceType"("type");

-- AddForeignKey
ALTER TABLE "Hero" ADD CONSTRAINT "Hero_accountId_fkey" FOREIGN KEY ("accountId") REFERENCES "Account"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Relic" ADD CONSTRAINT "Relic_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "RelicEffect" ADD CONSTRAINT "RelicEffect_relicId_fkey" FOREIGN KEY ("relicId") REFERENCES "Relic"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Spell" ADD CONSTRAINT "Spell_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "DeckCard" ADD CONSTRAINT "DeckCard_heroCardId_fkey" FOREIGN KEY ("heroCardId") REFERENCES "HeroCard"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Deck" ADD CONSTRAINT "Deck_npcId_fkey" FOREIGN KEY ("npcId") REFERENCES "Npc"("id") ON DELETE SET NULL ON UPDATE CASCADE;

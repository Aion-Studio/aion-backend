-- -------------------------------------------------------------
-- TablePlus 5.9.0(538)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-03-31 9:46:22.1790 AM
-- -------------------------------------------------------------


DROP TABLE IF EXISTS "public"."Action";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Action" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "region_name" text NOT NULL,
    "leyline_id" text,
    "quest_id" text,
    "description" text,
    "cost" int4 DEFAULT 0,
    "npc_id" text,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."ActionCompleted";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."ActionCompleted" (
    "id" text NOT NULL,
    "action_name" text NOT NULL,
    "hero_id" text NOT NULL,
    "updated_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "created_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "lootBox" jsonb NOT NULL DEFAULT '{}'::jsonb,
    "completed_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."ArmorEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."ArmorEffect" (
    "id" text NOT NULL,
    "amount" int4 NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Attributes";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Attributes" (
    "id" text NOT NULL,
    "strength" int4 NOT NULL,
    "agility" int4 NOT NULL,
    "intelligence" int4 NOT NULL,
    "exploration" int4 NOT NULL,
    "crafting" int4 NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."BaseStats";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."BaseStats" (
    "id" text NOT NULL,
    "level" int4 NOT NULL,
    "xp" int4 NOT NULL,
    "damageMin" int4 NOT NULL,
    "damageMax" int4 NOT NULL,
    "hit_points" int4 NOT NULL,
    "armor" int4 NOT NULL,
    "resilience" int4 NOT NULL DEFAULT 0,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Card";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."Nation";
CREATE TYPE "public"."Nation" AS ENUM ('Dusane', 'Aylen', 'Ironmark', 'Kelidor', 'Meta');
DROP TYPE IF EXISTS "public"."Rarity";
CREATE TYPE "public"."Rarity" AS ENUM ('Common', 'Magic', 'Epic', 'Legendary');
DROP TYPE IF EXISTS "public"."CardType";
CREATE TYPE "public"."CardType" AS ENUM ('Spell', 'Minion');

-- Table Definition
CREATE TABLE "public"."Card" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "nation" "public"."Nation" NOT NULL,
    "rarity" "public"."Rarity" NOT NULL,
    "tier" int4 NOT NULL,
    "imgUrl" text NOT NULL DEFAULT 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg'::text,
    "damage" int4 NOT NULL,
    "health" int4 NOT NULL,
    "manaCost" int4 NOT NULL,
    "cardType" "public"."CardType" NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."ChargeEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."ChargeEffect" (
    "id" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."DamageEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."DamageEffect" (
    "id" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."DamageEffectDetails";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."DamageType";
CREATE TYPE "public"."DamageType" AS ENUM ('Physical', 'Spell', 'Chaos');
DROP TYPE IF EXISTS "public"."TargetType";
CREATE TYPE "public"."TargetType" AS ENUM ('Minion', 'Hero', 'Both');

-- Table Definition
CREATE TABLE "public"."DamageEffectDetails" (
    "id" text NOT NULL,
    "amount" int4 NOT NULL,
    "damageType" "public"."DamageType" NOT NULL,
    "targetType" "public"."TargetType" NOT NULL,
    "damageEffectId" text,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Deck";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Deck" (
    "id" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."DeckCard";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."DeckCard" (
    "id" text NOT NULL,
    "deckId" text NOT NULL,
    "cardId" text NOT NULL,
    "quantity" int4 NOT NULL DEFAULT 1,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Follower";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Follower" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "level" int4 NOT NULL,
    "attributes_id" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."FollowerTalent";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."FollowerTalent" (
    "followerId" text NOT NULL,
    "talentId" text NOT NULL,
    PRIMARY KEY ("followerId","talentId")
);

DROP TABLE IF EXISTS "public"."HealEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."TargetType";
CREATE TYPE "public"."TargetType" AS ENUM ('Minion', 'Hero', 'Both');

-- Table Definition
CREATE TABLE "public"."HealEffect" (
    "id" text NOT NULL,
    "amount" int4 NOT NULL,
    "targetType" "public"."TargetType" NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Hero";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Hero" (
    "id" text NOT NULL,
    "aionCapacity" int4 NOT NULL,
    "base_stats_id" text NOT NULL,
    "attributes_id" text NOT NULL,
    "inventory_id" text NOT NULL,
    "stamina" int4 NOT NULL DEFAULT 100,
    "staminaMax" int4 NOT NULL DEFAULT 100,
    "staminaRegenRate" int4 NOT NULL DEFAULT 1,
    "name" text NOT NULL DEFAULT 'Default name'::text,
    "last_stamina_regeneration_time" timestamp(3),
    "deckId" text,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."HeroActions";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."HeroActions" (
    "id" text NOT NULL,
    "hero_id" text NOT NULL,
    "action_id" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."HeroCard";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."HeroCard" (
    "id" text NOT NULL,
    "heroId" text NOT NULL,
    "cardId" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."HeroQuests";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."HeroQuests" (
    "id" text NOT NULL,
    "hero_id" text NOT NULL,
    "quest_id" text NOT NULL,
    "completed" bool NOT NULL DEFAULT false,
    "accepted" bool NOT NULL DEFAULT false,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."HeroRegion";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."HeroRegion" (
    "id" text NOT NULL,
    "hero_id" text NOT NULL,
    "region_name" text NOT NULL,
    "discovery_level" float8 NOT NULL,
    "current_location" bool NOT NULL DEFAULT false,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."HeroResource";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."HeroResource" (
    "id" text NOT NULL,
    "hero_id" text NOT NULL,
    "amount" int4 NOT NULL,
    "resource_type_id" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."HeroTalent";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."HeroTalent" (
    "heroId" text NOT NULL,
    "talentId" text NOT NULL,
    PRIMARY KEY ("heroId","talentId")
);

DROP TABLE IF EXISTS "public"."InitiativeEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."InitiativeEffect" (
    "id" text NOT NULL,
    "amount" int4 NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Inventory";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Inventory" (
    "id" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Item";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Item" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "weight" int4 NOT NULL,
    "value" int4 NOT NULL,
    "active_inventory_id" text,
    "backpack_inventory_id" text,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Leyline";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Leyline" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "xp_reward" int4 NOT NULL,
    "RegionName" text NOT NULL,
    "aion_rate" float8 NOT NULL,
    "discovery_required" int4 NOT NULL,
    "stamina_rate" float8 NOT NULL,
    "npcId" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."LifestealEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."LifestealEffect" (
    "id" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."MinionEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."MinionEffect" (
    "id" text NOT NULL,
    "cardId" text NOT NULL,
    "duration" int4 NOT NULL DEFAULT 0,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."MinionEffectEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."MinionEffectEffect" (
    "id" text NOT NULL,
    "minionEffectId" text NOT NULL,
    "duration" int4 NOT NULL DEFAULT 0,
    "tauntEffectId" text,
    "chargeEffectId" text,
    "lifestealEffectId" text,
    "pickupEffectId" text,
    "summonEffectId" text,
    "resilienceEffectId" text,
    "poisonEffectId" text,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Npc";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Npc" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "level" int4 NOT NULL,
    "resilience" int4 NOT NULL,
    "strength" int4 NOT NULL,
    "agility" int4 NOT NULL,
    "hp" int4 NOT NULL DEFAULT 150,
    "damageMax" int4 NOT NULL DEFAULT 0,
    "damageMin" int4 NOT NULL DEFAULT 0,
    "armor" int4 NOT NULL DEFAULT 0,
    "deckId" text,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."NpcCard";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."NpcCard" (
    "id" text NOT NULL,
    "npcId" text NOT NULL,
    "cardId" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."PickupEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."PickupEffect" (
    "id" text NOT NULL,
    "amount" int4 NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."PoisonEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."PoisonEffect" (
    "id" text NOT NULL,
    "amount" int4 NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Quest";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Quest" (
    "id" text NOT NULL,
    "title" text NOT NULL,
    "region_name" text NOT NULL,
    "questNumber" int4 NOT NULL,
    "cost" int4 NOT NULL DEFAULT 0,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Region";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Region" (
    "name" text NOT NULL,
    "adjacent_regions" _text,
    PRIMARY KEY ("name")
);

DROP TABLE IF EXISTS "public"."ResilienceEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."ResilienceEffect" (
    "id" text NOT NULL,
    "amount" int4 NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."ResourceCost";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."ResourceEnum";
CREATE TYPE "public"."ResourceEnum" AS ENUM ('Aion', 'Valor', 'NexusOrb', 'StormShard');

-- Table Definition
CREATE TABLE "public"."ResourceCost" (
    "id" text NOT NULL,
    "amount" int4 NOT NULL,
    "resource" "public"."ResourceEnum" NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."ResourceType";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."ResourceEnum";
CREATE TYPE "public"."ResourceEnum" AS ENUM ('Aion', 'Valor', 'NexusOrb', 'StormShard');

-- Table Definition
CREATE TABLE "public"."ResourceType" (
    "id" text NOT NULL,
    "type" "public"."ResourceEnum" NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."RetinueSlot";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."RetinueSlot" (
    "id" text NOT NULL,
    "slotType" text NOT NULL,
    "hero_id" text NOT NULL,
    "followerId" text,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."SpellEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."SpellEffect" (
    "id" text NOT NULL,
    "cardId" text NOT NULL,
    "duration" int4 NOT NULL DEFAULT 0,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."SpellEffectEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."SpellEffectEffect" (
    "id" text NOT NULL,
    "spellEffectId" text NOT NULL,
    "healEffectId" text,
    "armorEffectId" text,
    "resilienceEffectId" text,
    "poisonEffectId" text,
    "initiativeEffectId" text,
    "stunEffectId" text,
    "damageEffectId" text,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."StunEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."StunEffect" (
    "id" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."SummonEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."SummonEffect" (
    "id" text NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."Talent";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."Talent" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "description" text,
    "cooldown" int4 NOT NULL,
    "effects" jsonb NOT NULL,
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."TauntEffect";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."TauntEffect" (
    "id" text NOT NULL,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Action" ("id", "name", "region_name", "leyline_id", "quest_id", "description", "cost", "npc_id") VALUES
('42eeec71-3d4b-4f64-a46d-c33f5087b712', 'Raid', 'Dusane', NULL, 'be885fb0-4f60-4f9d-9a75-ceb4160a7f6a', 'Fight the monster in forrest', 0, 'ef5666e9-95ee-4097-a643-afef984efee5'),
('ebc7b02e-91c8-4d8c-a931-2c3ca3f4d74b', 'Explore', 'Dusane', NULL, 'be885fb0-4f60-4f9d-9a75-ceb4160a7f6a', NULL, 6, NULL);

INSERT INTO "public"."ActionCompleted" ("id", "action_name", "hero_id", "updated_at", "created_at", "lootBox", "completed_at") VALUES
('0a81a125-9c41-4117-998f-e407301adda9', 'Explore', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '2024-03-18 05:55:30.633', '2024-03-14 05:58:08.766', '{"result": {"xp": 21, "hero_id": "e75aba73-f132-4d1f-bab2-f16fd3518f86", "resources": [{"amount": 15, "resource": "StormShard"}], "created_time": null, "discovery_level_increase": 3.21}, "actionName": "Explore"}', '2024-03-14 05:58:08.766'),
('391af28c-ef7a-49e6-925c-33e2ace19bd2', 'Explore', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '2024-03-31 07:39:16.244', '2024-03-18 05:55:30.742', '{"result": {"xp": 23, "hero_id": "e75aba73-f132-4d1f-bab2-f16fd3518f86", "resources": [{"amount": 16, "resource": "StormShard"}], "created_time": null, "discovery_level_increase": 1.05}, "actionName": "Explore"}', '2024-03-18 05:55:30.742'),
('3cff79b2-daeb-4845-a054-d48de8bddbab', 'Explore', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '2024-03-02 16:09:16.341', '2024-03-02 16:09:14.981', '{"result": {"xp": 32, "hero_id": "e75aba73-f132-4d1f-bab2-f16fd3518f86", "resources": [{"amount": 16, "resource": "StormShard"}], "created_time": null, "discovery_level_increase": 1.13}, "actionName": "Explore"}', '2024-03-02 16:09:14.981'),
('6bc4c119-d2d0-4207-bd52-4c04e6b4defd', 'Explore', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '2024-02-21 07:43:52.162', '2024-02-21 07:43:46.07', '{"result": {"xp": 29, "hero_id": "e75aba73-f132-4d1f-bab2-f16fd3518f86", "resources": [{"amount": 5, "resource": "StormShard"}], "created_time": null, "discovery_level_increase": 4.52}, "actionName": "Explore"}', '2024-02-21 07:43:46.07'),
('7030124f-e207-4a88-81cc-141938c3595c', 'Explore', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '2024-03-02 16:09:14.318', '2024-03-02 16:08:50.587', '{"result": {"xp": 29, "hero_id": "e75aba73-f132-4d1f-bab2-f16fd3518f86", "resources": [{"amount": 8, "resource": "StormShard"}], "created_time": null, "discovery_level_increase": 4.44}, "actionName": "Explore"}', '2024-03-02 16:08:50.588'),
('8b9622cd-5272-4378-9a9b-e835ce4f24a3', 'Explore', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '2024-03-02 16:09:14.983', '2024-03-02 16:09:14.324', '{"result": {"xp": 28, "hero_id": "e75aba73-f132-4d1f-bab2-f16fd3518f86", "resources": [{"amount": 13, "resource": "StormShard"}], "created_time": null, "discovery_level_increase": 1.11}, "actionName": "Explore"}', '2024-03-02 16:09:14.324'),
('c122f423-ca22-4b3a-8ef2-c4081592e8ca', 'Explore', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '2024-03-02 16:09:29.354', '2024-03-02 16:09:25.324', '{"result": {"xp": 30, "hero_id": "e75aba73-f132-4d1f-bab2-f16fd3518f86", "resources": [{"amount": 12, "resource": "StormShard"}], "created_time": null, "discovery_level_increase": 4.48}, "actionName": "Explore"}', '2024-03-02 16:09:25.324'),
('e63bee70-3af4-4aa8-ba27-5b15e5a6f212', 'Explore', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '2024-03-02 16:08:50.484', '2024-02-21 07:43:52.157', '{"result": {"xp": 17, "hero_id": "e75aba73-f132-4d1f-bab2-f16fd3518f86", "resources": [{"amount": 12, "resource": "StormShard"}], "created_time": null, "discovery_level_increase": 2.16}, "actionName": "Explore"}', '2024-02-21 07:43:52.157'),
('f22d49a1-070b-4059-a21b-a6dd94bfa191', 'Explore', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '2024-03-14 05:58:08.681', '2024-03-02 16:09:29.353', '{"result": {"xp": 17, "hero_id": "e75aba73-f132-4d1f-bab2-f16fd3518f86", "resources": [{"amount": 10, "resource": "StormShard"}], "created_time": null, "discovery_level_increase": 4.12}, "actionName": "Explore"}', '2024-03-02 16:09:29.353'),
('f331cf3f-0dd0-40a2-899b-bcec9c6d6ac7', 'Explore', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '2024-03-02 16:09:25.331', '2024-03-02 16:09:16.335', '{"result": {"xp": 22, "hero_id": "e75aba73-f132-4d1f-bab2-f16fd3518f86", "resources": [{"amount": 13, "resource": "StormShard"}], "created_time": null, "discovery_level_increase": 1.09}, "actionName": "Explore"}', '2024-03-02 16:09:16.335');

INSERT INTO "public"."ArmorEffect" ("id", "amount") VALUES
('0b9964d7-a433-4a32-a2f6-7d0f652ba525', -10),
('7a83aac5-759b-45d2-ac50-3ef5bb46b034', -1),
('a7f88339-ed50-4d14-a75f-755fa67014a8', 2);

INSERT INTO "public"."Attributes" ("id", "strength", "agility", "intelligence", "exploration", "crafting") VALUES
('20d70fd0-c57c-4784-b401-12e788444eb7', 7, 1, 18, 18, 6),
('5756ea3d-d3b9-406f-9418-f571c0e6e7b5', 9, 18, 8, 9, 11),
('68bbfe10-d82c-43b7-ada3-4da70089f180', 18, 11, 18, 19, 18),
('9b7e233b-66b6-4e2f-9602-547963b2aa94', 9, 20, 17, 18, 19),
('debe7b0e-15c3-4f56-891e-aa0d96c00c9a', 5, 6, 4, 2, 5);

INSERT INTO "public"."BaseStats" ("id", "level", "xp", "damageMin", "damageMax", "hit_points", "armor", "resilience") VALUES
('1f3c451b-ff41-45f4-a237-27f7b8cadbe0', 1, 0, 24, 35, 626, 3, 0),
('92f1cd1d-a23d-4e07-ab04-be7987d589c8', 1, 0, 4, 6, 91, 8, 0),
('9e730d95-79b6-44a6-801a-217bbd503dd5', 1, 0, 4, 6, 102, 5, 0),
('cf496a19-39bd-49e7-af1b-eba156d02921', 1, 85, 24, 43, 30, 3, 0),
('d0bd8ea1-29a4-4cc5-840a-26f4d6120c02', 1, 0, 4, 6, 97, 14, 0);

INSERT INTO "public"."Card" ("id", "name", "nation", "rarity", "tier", "imgUrl", "damage", "health", "manaCost", "cardType") VALUES
('014251a9-29b5-4d6d-b6c6-00f4db0f9487', 'Pulverize', 'Ironmark', 'Common', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 3, 'Spell'),
('042421ac-4281-402c-817b-12c140a81c63', 'Mystic Barrier', 'Ironmark', 'Magic', 2, 'https://example.com/mysticbarrier.jpg', 0, 0, 4, 'Spell'),
('071053a5-dc47-4a9a-88a3-ba8b8f3b4467', 'Spellbreaker', 'Aylen', 'Common', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 5, 3, 3, 'Minion'),
('1857c897-8f56-4089-bfeb-d74fd4f24932', 'Chaos Bolt', 'Kelidor', 'Epic', 3, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 5, 'Spell'),
('196ef607-7e42-4cc8-9c02-021cf2e35940', 'Frostfire Bolt', 'Ironmark', 'Magic', 2, 'https://example.com/frostfirebolt.jpg', 0, 0, 3, 'Spell'),
('1f686337-a1da-4610-a212-39f372c4cbfc', 'Sylvan Archer', 'Dusane', 'Common', 1, 'https://example.com/sylvanarcher.jpg', 2, 3, 2, 'Minion'),
('474a2db2-3b0b-4fe2-ae97-23317e093bb6', 'Frostfire Bolt', 'Dusane', 'Magic', 2, 'https://example.com/frostfirebolt.jpg', 0, 0, 3, 'Spell'),
('4aaef1e6-2791-4bfe-989c-9b90c5d29c21', 'Thunderclap', 'Aylen', 'Magic', 2, 'https://example.com/thunderclap.jpg', 0, 0, 3, 'Spell'),
('4bad53dc-4d51-4c4d-a913-22363ea634dc', 'Blood Berserker', 'Kelidor', 'Legendary', 3, 'https://example.com/bloodberserker.jpg', 6, 8, 6, 'Minion'),
('551d2476-8302-4926-9146-c3cb504cb8b3', 'Cursed Witch', 'Kelidor', 'Legendary', 3, 'https://example.com/cursedwitch.jpg', 3, 4, 6, 'Minion'),
('5787fcc9-a84d-4697-b9e7-dcccb6d495f3', 'Healing Light', 'Aylen', 'Common', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 5, 'Spell'),
('585fd99d-001c-4262-b625-04b8b948fc3a', 'Revitalize', 'Ironmark', 'Magic', 2, 'https://example.com/revitalize.jpg', 0, 0, 2, 'Spell'),
('6497850d-80c7-4f17-b78a-f6280583d902', 'Lesser Cleanse', 'Aylen', 'Common', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 1, 'Spell'),
('662e099f-5d45-46b5-bed0-148085e58904', 'Death Pact', 'Kelidor', 'Common', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 4, 'Spell'),
('75e09532-eced-4cdf-9c05-84deeb580929', 'Blizzard', 'Dusane', 'Common', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 4, 'Spell'),
('7bd69477-25fa-47e3-afc6-ec023faa6152', 'Warg Rider', 'Ironmark', 'Epic', 3, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 7, 5, 5, 'Minion'),
('80da753b-7d1c-4d78-9c8b-403aed492d09', 'Shadowmeld Assassin', 'Aylen', 'Legendary', 4, 'https://example.com/shadowmeldassassin.jpg', 8, 5, 8, 'Minion'),
('854d1b68-ea54-4020-bcc3-a871ca422003', 'Spellbreaker', 'Aylen', 'Common', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 5, 3, 3, 'Minion'),
('86118326-4107-4ffe-95ae-9f3b21635980', 'Frostfire Bolt', 'Dusane', 'Magic', 2, 'https://example.com/frostfirebolt.jpg', 0, 0, 3, 'Spell'),
('903f04a9-5bf8-474a-9fe4-02321540070c', 'Mystic Barrier', 'Aylen', 'Magic', 2, 'https://example.com/mysticbarrier.jpg', 0, 0, 4, 'Spell'),
('9abd0ddf-2b4d-4b42-9663-ca4447fe31b5', 'Lordaeron Footman', 'Dusane', 'Common', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 1, 2, 1, 'Minion'),
('9ca11be8-1e9e-4889-8ad6-75353a275577', 'Divine Protector', 'Kelidor', 'Epic', 3, 'https://example.com/divineprotector.jpg', 5, 9, 7, 'Minion'),
('9db9cf95-1a28-470b-8641-6a49d5da9ac7', 'Ethereal Phantom', 'Ironmark', 'Epic', 3, 'https://example.com/etherealphantom.jpg', 6, 4, 5, 'Minion'),
('a226626e-3737-4044-a5b5-29d3cd2f6c2d', 'Mountain King', 'Ironmark', 'Legendary', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 12, 3, 9, 'Minion'),
('a400daa6-1079-4781-b3c4-10c12b94f43e', 'Lordaeron Footman', 'Dusane', 'Common', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 1, 2, 1, 'Minion'),
('a6864f7f-8189-4c2b-945f-073c330890de', 'Cleanse', 'Aylen', 'Common', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 4, 'Spell'),
('aa0c8a92-636f-413a-a88a-91fe5f10fa5c', 'Mountain King', 'Ironmark', 'Legendary', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 8, 6, 9, 'Minion'),
('ab6243ee-b9cc-408a-9d6c-a0acf87a184d', 'Mountain King', 'Dusane', 'Common', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 9, 5, 9, 'Minion'),
('af068e40-0026-4a15-8676-a6010d4d6c63', 'Heaven''s Fury', 'Aylen', 'Legendary', 4, 'https://example.com/heavensfury.jpg', 0, 0, 9, 'Spell'),
('b46ed5f9-2539-482d-a7e9-67e1e3f277a8', 'Decimate', 'Ironmark', 'Epic', 4, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 8, 'Spell'),
('b473cd1a-0370-4971-8db5-c5a7f007fee1', 'Master Tactics', 'Dusane', 'Common', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 1, 'Spell'),
('b819a7b6-efe0-43ea-8a35-f01c4eb2438c', 'Mountain King', 'Ironmark', 'Legendary', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 7, 7, 9, 'Minion'),
('c0f8326b-315f-4292-a946-1aee9e861130', 'Goblin Sapper', 'Meta', 'Common', 2, 'https://example.com/goblinsapper.jpg', 4, 2, 3, 'Minion'),
('cadb1e00-9879-4e38-96d8-e4f8720d6c0a', 'Thunderclap', 'Kelidor', 'Magic', 2, 'https://example.com/thunderclap.jpg', 0, 0, 3, 'Spell'),
('d13ea357-fb95-439c-85e4-85e2f95ef066', 'Shadowstrike', 'Kelidor', 'Epic', 3, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 5, 'Spell'),
('d600bb1c-e6ef-4aa6-8b12-d79136dc11c7', 'Shieldwall', 'Dusane', 'Common', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/d1fdf798-a967-4a89-325f-22432cf29f00/lg', 0, 0, 2, 'Spell'),
('d8c10d6c-b95e-4985-8203-55f51f34e77a', 'Goblin Sapper', 'Ironmark', 'Common', 2, 'https://example.com/goblinsapper.jpg', 4, 2, 3, 'Minion'),
('dfd5f97e-c311-4c41-869a-dfc60c436943', 'Arcane Enlightenment', 'Aylen', 'Magic', 2, 'https://example.com/arcaneenlightenment.jpg', 0, 0, 2, 'Spell'),
('ef4fdea3-9dd0-4085-ac5d-7e55a852db51', 'Ironclad Defender', 'Ironmark', 'Epic', 3, 'https://example.com/ironcladdefender.jpg', 2, 6, 4, 'Minion'),
('f082f192-b492-4814-9f9c-5cd700de6bde', 'Nature''s Wrath', 'Aylen', 'Epic', 3, 'https://example.com/natureswrath.jpg', 0, 0, 5, 'Spell'),
('f6267068-cf71-44c2-ba2d-97d46ffa21c6', 'Heaven''s Fury', 'Meta', 'Legendary', 4, 'https://example.com/heavensfury.jpg', 0, 0, 9, 'Spell'),
('f897122d-0ee6-40d8-81e2-9f07da42cbdf', 'Ethereal Phantom', 'Aylen', 'Epic', 3, 'https://example.com/etherealphantom.jpg', 6, 4, 5, 'Minion'),
('f992ea2f-f0ca-4f72-8780-d1a2385efd89', 'Sylvan Archer', 'Kelidor', 'Common', 1, 'https://example.com/sylvanarcher.jpg', 2, 3, 2, 'Minion');

INSERT INTO "public"."ChargeEffect" ("id") VALUES
('7fa4fb39-8dff-4a30-ba0c-626b7504a498');

INSERT INTO "public"."DamageEffect" ("id") VALUES
('2b1e3114-15be-4197-82c3-05c6a6ba2a9c'),
('420faa96-f8ca-4f68-983a-3474d93871f3'),
('6b5cd58a-21f4-439c-92da-51899a501fdd'),
('85914e79-74cf-4d70-ba25-ec6c57a1403f'),
('955e47c3-ee6d-420e-94ef-e21d026c386d'),
('a31b37fa-67f0-46d8-8ec2-124c1d351ea4'),
('ab9abffd-212e-4042-9628-e1c13f823189'),
('acc3d9ab-c15d-42cf-b380-1a420aba2c5b'),
('b0b9d960-1ca0-4a6d-af4c-029394c65451'),
('b804560a-e3cc-44dd-b983-3ff2f0dd9a63'),
('bad3c3c0-342f-4c75-b6b4-1a1917b68bfb'),
('d2e9c70f-56c0-4e9a-b48e-83c12b8c9a73'),
('d3aa02f0-5e01-43e4-8860-f34c30b4f4f6'),
('d7dff637-aecf-4147-85e9-31454c6ea58d'),
('dd7aa94d-6646-4f58-9552-8be95deeb6a9'),
('e2b5fd57-b8bc-4324-a30d-e53fe1cb51c3'),
('f7e75913-787a-4dbb-adb2-3b83530a272e');

INSERT INTO "public"."DamageEffectDetails" ("id", "amount", "damageType", "targetType", "damageEffectId") VALUES
('61cea58e-eb33-4485-b702-4d63bd5bae8e', 3, 'Physical', 'Minion', 'd7dff637-aecf-4147-85e9-31454c6ea58d'),
('70b191a4-8946-433b-bbd7-b8a62262a9b9', 1, 'Physical', 'Minion', '420faa96-f8ca-4f68-983a-3474d93871f3'),
('d4a10717-f46c-4ff1-a47d-1f96e0398651', 2, 'Physical', 'Hero', 'dd7aa94d-6646-4f58-9552-8be95deeb6a9');

INSERT INTO "public"."Deck" ("id") VALUES
('77300e4b-fa14-4488-81c4-5b30cbc58ca1'),
('a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65'),
('fc82bc0d-0973-4faf-831b-5a0ad0a29930');

INSERT INTO "public"."DeckCard" ("id", "deckId", "cardId", "quantity") VALUES
('01bb4a05-9300-4049-a6de-aa2a94802a6e', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930', '9abd0ddf-2b4d-4b42-9663-ca4447fe31b5', 1),
('0a60cd9e-92e1-4aa7-9490-72b95cfa94ae', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', '1f686337-a1da-4610-a212-39f372c4cbfc', 1),
('0c770054-d8a4-4ab2-82fe-505a4287796f', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930', 'b473cd1a-0370-4971-8db5-c5a7f007fee1', 1),
('0e528ce1-bf4a-4f17-b9f5-47a115d2b7e3', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', 'b473cd1a-0370-4971-8db5-c5a7f007fee1', 1),
('1cdcc63e-c233-4f75-a371-4f25df2bad94', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', 'ab6243ee-b9cc-408a-9d6c-a0acf87a184d', 1),
('252cef17-ecac-4fc5-a4c0-1325c1276dbe', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', 'cadb1e00-9879-4e38-96d8-e4f8720d6c0a', 1),
('331d7ddc-8df7-41da-a1f3-9e3fade8d355', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', '7bd69477-25fa-47e3-afc6-ec023faa6152', 1),
('339f4d85-7889-4a26-bbb0-272fe216f6e7', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930', 'ab6243ee-b9cc-408a-9d6c-a0acf87a184d', 1),
('4a3625d8-af2a-4906-97d2-0e193e32b8d5', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', '903f04a9-5bf8-474a-9fe4-02321540070c', 1),
('5eae56ed-49e7-42ce-a045-abc3cc6c9be6', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930', '854d1b68-ea54-4020-bcc3-a871ca422003', 1),
('63ac61e1-475a-4cb3-be4c-907d7f16d959', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', '86118326-4107-4ffe-95ae-9f3b21635980', 1),
('64dbbf89-748c-4fc2-89ce-42e419ebb676', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', 'b46ed5f9-2539-482d-a7e9-67e1e3f277a8', 1),
('6ef8ee8e-656e-47c4-8edd-cef20bf15aae', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930', '071053a5-dc47-4a9a-88a3-ba8b8f3b4467', 1),
('85928956-33be-4fa5-bbf8-23e7f90a8515', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930', 'aa0c8a92-636f-413a-a88a-91fe5f10fa5c', 1),
('85fce2a6-d71f-4a5f-8c81-8763675297c5', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930', 'a6864f7f-8189-4c2b-945f-073c330890de', 1),
('8f5be329-ef9a-4fa6-9526-dc1728488b1d', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', '042421ac-4281-402c-817b-12c140a81c63', 1),
('a190dc05-0383-4ca5-90d9-9e135469717f', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930', 'a226626e-3737-4044-a5b5-29d3cd2f6c2d', 1),
('a9acde6c-4327-4677-a074-8ba1c8e6c5b7', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930', 'a400daa6-1079-4781-b3c4-10c12b94f43e', 1),
('b1573174-7431-4eeb-905e-b5e55b6ab1e6', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', 'dfd5f97e-c311-4c41-869a-dfc60c436943', 1),
('d542e696-877b-464f-81ea-e8848a366eaf', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930', 'b819a7b6-efe0-43ea-8a35-f01c4eb2438c', 1),
('e75723e7-087e-4c16-8e82-034fa7d1b93e', 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65', 'f6267068-cf71-44c2-ba2d-97d46ffa21c6', 1);

INSERT INTO "public"."HealEffect" ("id", "amount", "targetType") VALUES
('32ac5d87-b675-45df-85ba-6e79c628edae', 2, 'Hero'),
('4fd490b1-a75a-43cd-8aee-5d9facd8f218', 5, 'Both');

INSERT INTO "public"."Hero" ("id", "aionCapacity", "base_stats_id", "attributes_id", "inventory_id", "stamina", "staminaMax", "staminaRegenRate", "name", "last_stamina_regeneration_time", "deckId") VALUES
('e75aba73-f132-4d1f-bab2-f16fd3518f86', 96, 'cf496a19-39bd-49e7-af1b-eba156d02921', '9b7e233b-66b6-4e2f-9602-547963b2aa94', '307121e5-cb78-4ec1-9eed-10737094fcf5', 100, 100, 1, 'NoZoUr', '2024-03-31 07:39:16.244', 'fc82bc0d-0973-4faf-831b-5a0ad0a29930'),
('fe4c402b-8128-4f0a-9218-1bac2d14beaf', 87, '1f3c451b-ff41-45f4-a237-27f7b8cadbe0', '68bbfe10-d82c-43b7-ada3-4da70089f180', '6a795430-e8a3-4ae2-8c3c-05e044c6cd82', 100, 100, 1, 'IlNa', NULL, '77300e4b-fa14-4488-81c4-5b30cbc58ca1');

INSERT INTO "public"."HeroCard" ("id", "heroId", "cardId") VALUES
('3cd72aeb-058e-4469-8356-dd73cda494dc', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '474a2db2-3b0b-4fe2-ae97-23317e093bb6'),
('4798cab2-870c-4dc6-a308-e0fad9645f1f', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '042421ac-4281-402c-817b-12c140a81c63'),
('49940d29-de15-45d3-b4dd-4d617e57b095', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', 'f6267068-cf71-44c2-ba2d-97d46ffa21c6'),
('5b61c033-685f-4c0d-9e2b-79d37e347542', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '585fd99d-001c-4262-b625-04b8b948fc3a'),
('5c809538-5b53-48b0-a1be-28b389641ff3', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '014251a9-29b5-4d6d-b6c6-00f4db0f9487'),
('7306fc18-b9dd-4ea2-9bec-0dbfa6614ad2', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '474a2db2-3b0b-4fe2-ae97-23317e093bb6'),
('76246b7e-d477-4872-a6ea-8dce658b3dd5', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', 'b819a7b6-efe0-43ea-8a35-f01c4eb2438c'),
('7ccb1b48-9a86-4eec-b41b-ce317d002e34', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', 'aa0c8a92-636f-413a-a88a-91fe5f10fa5c'),
('7f2186a3-70b7-4c94-8962-ba32c9565713', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '662e099f-5d45-46b5-bed0-148085e58904'),
('8e0ff25a-6707-4141-aa87-306383d85f72', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '014251a9-29b5-4d6d-b6c6-00f4db0f9487'),
('d24602f7-c285-48d9-9c47-a39140cec925', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', 'a400daa6-1079-4781-b3c4-10c12b94f43e'),
('d5c070ce-8506-43ab-bb34-ca4b80c19d33', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '071053a5-dc47-4a9a-88a3-ba8b8f3b4467'),
('f69633d7-05a7-4953-9962-f0e3495de1bf', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', '9abd0ddf-2b4d-4b42-9663-ca4447fe31b5');

INSERT INTO "public"."HeroQuests" ("id", "hero_id", "quest_id", "completed", "accepted") VALUES
('4bd6a093-d857-4af6-99da-36bd8307e458', 'fe4c402b-8128-4f0a-9218-1bac2d14beaf', 'be885fb0-4f60-4f9d-9a75-ceb4160a7f6a', 'f', 'f'),
('6740bb2e-6465-4750-b3c6-d52468008524', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', 'be885fb0-4f60-4f9d-9a75-ceb4160a7f6a', 'f', 't');

INSERT INTO "public"."HeroRegion" ("id", "hero_id", "region_name", "discovery_level", "current_location") VALUES
('05948fe3-0f4e-48f5-9037-3c00fed9d1c8', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', 'Dusane', 27.31, 't'),
('bac8ab88-bcb9-4cfa-a6cb-65775a3ebc75', 'fe4c402b-8128-4f0a-9218-1bac2d14beaf', 'Dusane', 0, 't');

INSERT INTO "public"."HeroResource" ("id", "hero_id", "amount", "resource_type_id") VALUES
('0d7bc4ff-be2c-4193-b961-0bb183c4e562', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', 15, '4b92d9a3-8d30-4895-babb-066ef85a3b4c'),
('a2ecf7ac-8d71-4e71-ab40-ae5830700b21', 'e75aba73-f132-4d1f-bab2-f16fd3518f86', 32, '64bb06f5-04e4-42d9-97f0-28f31f89b370');

INSERT INTO "public"."InitiativeEffect" ("id", "amount") VALUES
('0c87cbf1-e8bb-499d-87de-18b601f6b56e', 1),
('3af27a3c-c105-44d3-b11a-90658e3336c3', 1),
('c52f2b31-de2f-4989-9f4a-32d5b846edb9', 1);

INSERT INTO "public"."Inventory" ("id") VALUES
('0b5517a7-48c9-4df3-b5c7-455f72a834ea'),
('307121e5-cb78-4ec1-9eed-10737094fcf5'),
('3afc00ca-ec3f-4e1a-8a07-97b20b93d4cc'),
('6a795430-e8a3-4ae2-8c3c-05e044c6cd82'),
('80cc0aa3-c1d5-435d-aeed-d814ed21bfe1');

INSERT INTO "public"."LifestealEffect" ("id") VALUES
('04df37fb-811c-4404-8898-7840f986267a'),
('0e60f167-3cc9-489e-bfd2-4c1eba2b97b9'),
('3cae83e9-ec0a-420f-84f7-d818f31437c1'),
('79670a1c-ea13-41be-80f3-2e4c72908a3e');

INSERT INTO "public"."MinionEffect" ("id", "cardId", "duration") VALUES
('0ce8e6f5-ff98-42fd-821e-10c7acba6fff', '1f686337-a1da-4610-a212-39f372c4cbfc', 0),
('1534ba4a-4ef4-4663-8dce-490e658d3361', '071053a5-dc47-4a9a-88a3-ba8b8f3b4467', 0),
('1e965454-5728-4b91-8d67-dcdb1512cd29', 'ef4fdea3-9dd0-4085-ac5d-7e55a852db51', 0),
('23a355fc-5395-4bad-b1f3-62fc8d06e412', '7bd69477-25fa-47e3-afc6-ec023faa6152', 0),
('3458591b-b5b8-41fe-a3fb-fd1af8cf6aba', 'f897122d-0ee6-40d8-81e2-9f07da42cbdf', 0),
('69cb2f8c-c78b-475b-89cb-9e7083bb8177', '9ca11be8-1e9e-4889-8ad6-75353a275577', 0),
('69e0ba2e-4427-4469-b934-d46efcd05896', 'd8c10d6c-b95e-4985-8203-55f51f34e77a', 0),
('84eef881-4e50-444d-ad98-49a609fd4d8b', 'a226626e-3737-4044-a5b5-29d3cd2f6c2d', 0),
('9375b40d-2bc1-46b1-a72c-4a64794ac788', 'd8c10d6c-b95e-4985-8203-55f51f34e77a', 0),
('b2cf00f8-ddc9-4ebc-b4cb-c9acf05e579b', '4bad53dc-4d51-4c4d-a913-22363ea634dc', 0),
('b482c366-30d7-4d3d-9d36-618252d42523', '551d2476-8302-4926-9146-c3cb504cb8b3', 0),
('d4f8ebc1-3857-4c49-9c3f-49cd493fd1e8', '854d1b68-ea54-4020-bcc3-a871ca422003', 0),
('d963d34e-6837-4a44-8802-a03029bb60d6', '80da753b-7d1c-4d78-9c8b-403aed492d09', 0),
('ea0958f4-6598-4517-81fe-779cad342096', '9db9cf95-1a28-470b-8641-6a49d5da9ac7', 0);

INSERT INTO "public"."MinionEffectEffect" ("id", "minionEffectId", "duration", "tauntEffectId", "chargeEffectId", "lifestealEffectId", "pickupEffectId", "summonEffectId", "resilienceEffectId", "poisonEffectId") VALUES
('137ad73b-e36c-4764-9be9-c9e7f6217d71', 'ea0958f4-6598-4517-81fe-779cad342096', 0, '0e6a08a5-ccc1-4e6f-8345-32303f3a7c2b', NULL, NULL, NULL, NULL, NULL, NULL),
('146eb467-beff-4c44-bfb7-dd7341afad62', '69e0ba2e-4427-4469-b934-d46efcd05896', 0, NULL, NULL, '79670a1c-ea13-41be-80f3-2e4c72908a3e', NULL, NULL, NULL, NULL),
('74bc366c-48c7-4b03-8bb2-2d725003b4a4', 'b482c366-30d7-4d3d-9d36-618252d42523', 0, NULL, NULL, NULL, '48721d26-035b-482b-9895-dbd8017d7e03', NULL, NULL, NULL),
('7e9c653c-5d8d-4471-92b8-1548bc5410c2', '1534ba4a-4ef4-4663-8dce-490e658d3361', 0, NULL, NULL, NULL, 'e27a0116-1608-482d-a3dd-6526f104eecc', NULL, NULL, NULL),
('85d7d5ab-b076-4d3a-8453-8912e3aface1', '3458591b-b5b8-41fe-a3fb-fd1af8cf6aba', 0, NULL, NULL, '3cae83e9-ec0a-420f-84f7-d818f31437c1', NULL, NULL, NULL, NULL),
('87eb1bbb-6846-4306-8306-d5fde39e5a0b', '69cb2f8c-c78b-475b-89cb-9e7083bb8177', 0, NULL, NULL, '04df37fb-811c-4404-8898-7840f986267a', NULL, NULL, NULL, NULL),
('972e5e08-f055-4fd5-9677-f5c1c54fd617', 'd4f8ebc1-3857-4c49-9c3f-49cd493fd1e8', 0, NULL, NULL, '0e60f167-3cc9-489e-bfd2-4c1eba2b97b9', NULL, NULL, NULL, NULL),
('ae0a7ada-f5ef-404d-96a0-91441306efde', '23a355fc-5395-4bad-b1f3-62fc8d06e412', 0, NULL, NULL, NULL, NULL, NULL, NULL, 'c890f5e5-8bd7-4fd8-85fd-fa9e1892213e'),
('ce582eb9-1aa0-4a6c-bdfc-86a370e67212', '84eef881-4e50-444d-ad98-49a609fd4d8b', 0, NULL, NULL, NULL, NULL, NULL, '2b765e79-3a61-4a4a-b4cf-cec338df9ee4', NULL),
('dca8f959-4d19-4295-aa20-2dc691123545', '1e965454-5728-4b91-8d67-dcdb1512cd29', 0, NULL, NULL, '79670a1c-ea13-41be-80f3-2e4c72908a3e', NULL, NULL, NULL, NULL),
('defdd25f-ef29-41d2-a22f-e0df497eb76c', 'b2cf00f8-ddc9-4ebc-b4cb-c9acf05e579b', 0, NULL, '7fa4fb39-8dff-4a30-ba0c-626b7504a498', NULL, NULL, NULL, NULL, NULL),
('ec75a593-6535-4b4c-94f7-3181df3c67b6', 'd963d34e-6837-4a44-8802-a03029bb60d6', 0, '0e6a08a5-ccc1-4e6f-8345-32303f3a7c2b', NULL, NULL, NULL, NULL, NULL, NULL),
('ecef230f-db49-4d5d-837d-3573603ba5ee', '9375b40d-2bc1-46b1-a72c-4a64794ac788', 0, NULL, NULL, NULL, NULL, '58eea1bd-ff17-4a89-9748-a4218f32b0e7', NULL, NULL),
('f3e6db13-dd39-4a5b-ad27-808ea33e83c1', '0ce8e6f5-ff98-42fd-821e-10c7acba6fff', 0, NULL, NULL, NULL, NULL, NULL, NULL, 'e204ab08-14d4-4447-b5e3-355dd0c3eefb');

INSERT INTO "public"."Npc" ("id", "name", "level", "resilience", "strength", "agility", "hp", "damageMax", "damageMin", "armor", "deckId") VALUES
('ef5666e9-95ee-4097-a643-afef984efee5', 'Ragnor', 1, 2, 5, 2, 30, 14, 10, 2, 'a7427de5-f78c-4ab5-bf3b-4b36ef1a9d65');

INSERT INTO "public"."NpcCard" ("id", "npcId", "cardId") VALUES
('4326090f-0750-41aa-8bad-2479514b92ab', 'ef5666e9-95ee-4097-a643-afef984efee5', 'ab6243ee-b9cc-408a-9d6c-a0acf87a184d'),
('50bd97b9-dd01-41f5-9934-dc2204feb88b', 'ef5666e9-95ee-4097-a643-afef984efee5', '042421ac-4281-402c-817b-12c140a81c63'),
('5860aa75-c760-4e8d-99ce-5ff3ebc1fc79', 'ef5666e9-95ee-4097-a643-afef984efee5', 'b46ed5f9-2539-482d-a7e9-67e1e3f277a8'),
('5bed91da-1322-4b97-9f71-c1e2edcf6503', 'ef5666e9-95ee-4097-a643-afef984efee5', '903f04a9-5bf8-474a-9fe4-02321540070c'),
('75392e23-f9db-4186-85a6-4491f63e5ef3', 'ef5666e9-95ee-4097-a643-afef984efee5', 'f6267068-cf71-44c2-ba2d-97d46ffa21c6'),
('9b116282-197b-4461-9d6c-56bdbcd3efbd', 'ef5666e9-95ee-4097-a643-afef984efee5', '86118326-4107-4ffe-95ae-9f3b21635980'),
('d69ac158-8baa-41d8-b6b4-d74ab8a90e46', 'ef5666e9-95ee-4097-a643-afef984efee5', '1f686337-a1da-4610-a212-39f372c4cbfc'),
('e0a98f16-fe4c-47fc-8b3c-0772411ff0e0', 'ef5666e9-95ee-4097-a643-afef984efee5', 'dfd5f97e-c311-4c41-869a-dfc60c436943');

INSERT INTO "public"."PickupEffect" ("id", "amount") VALUES
('48721d26-035b-482b-9895-dbd8017d7e03', 2),
('e27a0116-1608-482d-a3dd-6526f104eecc', 2);

INSERT INTO "public"."PoisonEffect" ("id", "amount") VALUES
('08b99ec7-f295-4fc1-914a-8b22db47726b', 2),
('7d7a2b05-04ef-4a50-ae52-1cb507341e7e', 1),
('a8d98067-82e4-474a-8427-c94439004f19', 0),
('c2ad7be5-420f-4ccc-943b-37732fb3153b', 5),
('c890f5e5-8bd7-4fd8-85fd-fa9e1892213e', 2),
('e204ab08-14d4-4447-b5e3-355dd0c3eefb', 4);

INSERT INTO "public"."Quest" ("id", "title", "region_name", "questNumber", "cost") VALUES
('be885fb0-4f60-4f9d-9a75-ceb4160a7f6a', 'Obtain the medallion', 'Dusane', 1, 15);

INSERT INTO "public"."Region" ("name", "adjacent_regions") VALUES
('Buzna', '{Lindon}'),
('Dusane', '{Veladria}'),
('Emerlad', '{Veladria,Lindon,Yezer,Forest}'),
('Forest', '{Yezer,Emerlad}'),
('Lindon', '{Veladria,Lindon,Yezer,Buzna}'),
('Veladria', '{Dusane,Lindon,Emerlad}'),
('Yezer', '{Lindon,Emerlad,Forest}');

INSERT INTO "public"."ResilienceEffect" ("id", "amount") VALUES
('2b765e79-3a61-4a4a-b4cf-cec338df9ee4', 100000),
('306a9f2c-ce4a-4386-aeb5-e269fc9f5af3', 3),
('8e1fef81-6127-4443-b398-0de171aa9575', 2),
('ec465525-d83a-4cc2-93ca-2d53d4506cc3', 3),
('ee18f9e5-ca9a-41fb-bfd7-2324f3fff16a', 3);

INSERT INTO "public"."ResourceType" ("id", "type") VALUES
('105d5452-bfe0-44c6-893d-18cad6192eee', 'NexusOrb'),
('4b92d9a3-8d30-4895-babb-066ef85a3b4c', 'StormShard'),
('64bb06f5-04e4-42d9-97f0-28f31f89b370', 'Valor'),
('e1059285-3b37-4224-8bd9-5653cb8c3586', 'Aion');

INSERT INTO "public"."SpellEffect" ("id", "cardId", "duration") VALUES
('033af67d-8c1b-475b-a1df-141b6c887f95', 'ab6243ee-b9cc-408a-9d6c-a0acf87a184d', 1),
('0eeb8179-9b5d-4d01-ab0b-8d9c880e42e2', '86118326-4107-4ffe-95ae-9f3b21635980', 0),
('20253bbe-f166-4fcb-b7ee-ad1dda99f8d9', '585fd99d-001c-4262-b625-04b8b948fc3a', 0),
('27a9ca5a-3bde-42cc-a023-093ebc90d859', 'b473cd1a-0370-4971-8db5-c5a7f007fee1', 0),
('328da7e6-90fe-44d8-9ed0-2a04cbbdf14d', 'dfd5f97e-c311-4c41-869a-dfc60c436943', 0),
('36c63be8-eb03-447d-be10-fca1151dbc93', '75e09532-eced-4cdf-9c05-84deeb580929', 0),
('4231c960-8d2a-4fca-b149-6845c32dccda', 'af068e40-0026-4a15-8676-a6010d4d6c63', 0),
('45b2dd9a-d982-4326-8671-5a4850869553', '86118326-4107-4ffe-95ae-9f3b21635980', 0),
('59899f5a-f02e-4110-a7a1-1f59a97f69be', '5787fcc9-a84d-4697-b9e7-dcccb6d495f3', 0),
('5a55032e-91c7-46f5-bd24-1d508327c946', '014251a9-29b5-4d6d-b6c6-00f4db0f9487', 0),
('5ea23cdd-5c6e-42fd-9dde-0bbc682a46b0', '474a2db2-3b0b-4fe2-ae97-23317e093bb6', 0),
('6876bd57-8800-421a-8ece-2fb84ad7111a', 'b46ed5f9-2539-482d-a7e9-67e1e3f277a8', 0),
('6d950f07-0c90-4814-831c-d30b06d1942c', '474a2db2-3b0b-4fe2-ae97-23317e093bb6', 0),
('712de550-0f88-46e3-b74a-66098e4283d1', '6497850d-80c7-4f17-b78a-f6280583d902', 0),
('793db417-923f-4c58-acc1-cded7296ad66', '903f04a9-5bf8-474a-9fe4-02321540070c', 0),
('8a3d366f-4f90-494f-bb9f-3259be135328', 'd600bb1c-e6ef-4aa6-8b12-d79136dc11c7', 0),
('98a7913f-bc50-4a16-86a8-2f44dfe5602f', 'f6267068-cf71-44c2-ba2d-97d46ffa21c6', 0),
('b258075e-f0d6-47f9-a397-66c707acecb1', 'cadb1e00-9879-4e38-96d8-e4f8720d6c0a', 0),
('b4ffdb57-047a-4bab-8d72-baf5ef76a196', '196ef607-7e42-4cc8-9c02-021cf2e35940', 0),
('b912845d-af2b-494d-b232-51a35ae3667c', 'd13ea357-fb95-439c-85e4-85e2f95ef066', 0),
('bcc05be2-a695-4418-bb20-4b3c521a1018', '585fd99d-001c-4262-b625-04b8b948fc3a', 0),
('bce726e1-84b0-463b-806e-8cd5a52b776e', 'f6267068-cf71-44c2-ba2d-97d46ffa21c6', 0),
('c84efdc6-8d1f-47e5-959f-ae5dc989c84a', 'f6267068-cf71-44c2-ba2d-97d46ffa21c6', 0),
('d430d358-5319-493c-804c-8887adf09464', 'a6864f7f-8189-4c2b-945f-073c330890de', 0),
('d638162c-024b-49d4-a7a4-c85f1f44e228', '1857c897-8f56-4089-bfeb-d74fd4f24932', 0),
('db576cb7-dffb-4e42-bf32-b9707f98fad3', '4aaef1e6-2791-4bfe-989c-9b90c5d29c21', 0),
('dc304a66-2dd5-48b0-9664-e0510a6448c7', '662e099f-5d45-46b5-bed0-148085e58904', 0),
('dd95a7db-1b91-427e-98b4-f6461c725a45', 'f082f192-b492-4814-9f9c-5cd700de6bde', 0),
('e6f8262a-1945-4bb5-a79d-076183a9fcaf', '042421ac-4281-402c-817b-12c140a81c63', 0),
('efcd8428-80d2-45e3-9562-337a20c9da15', '4aaef1e6-2791-4bfe-989c-9b90c5d29c21', 0),
('fe5b2b52-3355-441e-b4ff-4b5ab76d8857', 'af068e40-0026-4a15-8676-a6010d4d6c63', 0);

INSERT INTO "public"."SpellEffectEffect" ("id", "spellEffectId", "healEffectId", "armorEffectId", "resilienceEffectId", "poisonEffectId", "initiativeEffectId", "stunEffectId", "damageEffectId") VALUES
('10f6cf2d-54b3-49ff-bdd5-f0be0fe2f522', '328da7e6-90fe-44d8-9ed0-2a04cbbdf14d', NULL, NULL, NULL, NULL, NULL, NULL, '2b1e3114-15be-4197-82c3-05c6a6ba2a9c'),
('18ff1a83-9504-401b-bb91-6f0386c7ac99', 'd638162c-024b-49d4-a7a4-c85f1f44e228', NULL, NULL, NULL, '7d7a2b05-04ef-4a50-ae52-1cb507341e7e', NULL, NULL, NULL),
('19168537-570b-4e84-b3bf-d348214eb09e', '59899f5a-f02e-4110-a7a1-1f59a97f69be', '4fd490b1-a75a-43cd-8aee-5d9facd8f218', NULL, NULL, NULL, NULL, NULL, NULL),
('21c268ae-de1b-4a7a-9347-969a83642095', 'fe5b2b52-3355-441e-b4ff-4b5ab76d8857', NULL, NULL, '8e1fef81-6127-4443-b398-0de171aa9575', NULL, NULL, NULL, NULL),
('227e9a77-2e0e-4c46-895e-c730a758d144', 'dc304a66-2dd5-48b0-9664-e0510a6448c7', NULL, NULL, NULL, NULL, NULL, NULL, '420faa96-f8ca-4f68-983a-3474d93871f3'),
('237c59c6-a269-4853-adde-4bfd1179f978', '6876bd57-8800-421a-8ece-2fb84ad7111a', NULL, '0b9964d7-a433-4a32-a2f6-7d0f652ba525', NULL, NULL, NULL, NULL, NULL),
('31a4a5d5-f014-450a-9003-2d34f1c721a7', 'bce726e1-84b0-463b-806e-8cd5a52b776e', NULL, NULL, NULL, NULL, NULL, 'd9b9d92c-86ba-4efd-939a-586f8c930bf9', NULL),
('359dd7c1-6249-44e5-bd55-0bec6985f694', 'dc304a66-2dd5-48b0-9664-e0510a6448c7', '32ac5d87-b675-45df-85ba-6e79c628edae', NULL, NULL, NULL, NULL, NULL, NULL),
('3db2aa4b-135d-4097-8533-98d97255c10e', '033af67d-8c1b-475b-a1df-141b6c887f95', NULL, '7a83aac5-759b-45d2-ac50-3ef5bb46b034', NULL, NULL, NULL, NULL, NULL),
('48088e01-6515-432c-85f8-f7f357d9ffd3', 'bcc05be2-a695-4418-bb20-4b3c521a1018', NULL, NULL, '8e1fef81-6127-4443-b398-0de171aa9575', NULL, NULL, NULL, NULL),
('4f13b939-f2d0-4022-9c04-a5a3f7eebb4a', '0eeb8179-9b5d-4d01-ab0b-8d9c880e42e2', NULL, NULL, NULL, NULL, '0c87cbf1-e8bb-499d-87de-18b601f6b56e', NULL, NULL),
('5b414efd-7939-4b99-8888-f57bea4be4d3', 'b258075e-f0d6-47f9-a397-66c707acecb1', NULL, NULL, NULL, NULL, 'c52f2b31-de2f-4989-9f4a-32d5b846edb9', NULL, NULL),
('793f9dd2-0f64-4f5c-b1bf-3a518a290da1', '36c63be8-eb03-447d-be10-fca1151dbc93', NULL, NULL, '8e1fef81-6127-4443-b398-0de171aa9575', NULL, NULL, NULL, NULL),
('79a4d424-6431-4947-8c88-c4e0b0617449', '5a55032e-91c7-46f5-bd24-1d508327c946', '32ac5d87-b675-45df-85ba-6e79c628edae', '7a83aac5-759b-45d2-ac50-3ef5bb46b034', NULL, NULL, NULL, NULL, NULL),
('918107e1-1371-4def-ac16-a5858e107f1c', '20253bbe-f166-4fcb-b7ee-ad1dda99f8d9', NULL, NULL, '306a9f2c-ce4a-4386-aeb5-e269fc9f5af3', NULL, NULL, NULL, NULL),
('9230073c-3adc-4c75-94d4-f6d70e9b9816', 'b912845d-af2b-494d-b232-51a35ae3667c', NULL, NULL, NULL, NULL, '3af27a3c-c105-44d3-b11a-90658e3336c3', NULL, NULL),
('9ec2835e-5506-4035-9e7e-0023fdedf577', '5ea23cdd-5c6e-42fd-9dde-0bbc682a46b0', NULL, NULL, NULL, NULL, 'c52f2b31-de2f-4989-9f4a-32d5b846edb9', NULL, NULL),
('a758e9aa-e429-4bd6-a602-9a6a5b3334d3', 'c84efdc6-8d1f-47e5-959f-ae5dc989c84a', NULL, NULL, NULL, NULL, NULL, NULL, '955e47c3-ee6d-420e-94ef-e21d026c386d'),
('aba9fcf4-3c29-41fd-804d-49eb60112f4c', 'e6f8262a-1945-4bb5-a79d-076183a9fcaf', NULL, NULL, 'ec465525-d83a-4cc2-93ca-2d53d4506cc3', NULL, NULL, NULL, NULL),
('b550a71b-9136-460a-bf8b-ba7907981e6d', '98a7913f-bc50-4a16-86a8-2f44dfe5602f', NULL, NULL, 'ee18f9e5-ca9a-41fb-bfd7-2324f3fff16a', NULL, NULL, NULL, NULL),
('b8a6fc33-7f75-47b4-8318-6719fe6ea6db', 'd430d358-5319-493c-804c-8887adf09464', NULL, NULL, NULL, 'c2ad7be5-420f-4ccc-943b-37732fb3153b', NULL, NULL, NULL),
('ba6fc913-8525-489e-ba3e-380f8950736c', '8a3d366f-4f90-494f-bb9f-3259be135328', NULL, 'a7f88339-ed50-4d14-a75f-755fa67014a8', NULL, NULL, NULL, NULL, NULL),
('bd61084f-0c6e-4373-89d6-c35e0bb69e30', 'dd95a7db-1b91-427e-98b4-f6461c725a45', NULL, NULL, NULL, NULL, NULL, 'cc00930f-cc36-45a7-9637-a56f60ff67c4', NULL),
('ca2ec2ee-8156-4926-a8bd-aee84c76d1ac', '793db417-923f-4c58-acc1-cded7296ad66', NULL, NULL, '306a9f2c-ce4a-4386-aeb5-e269fc9f5af3', NULL, NULL, NULL, NULL),
('cf37e54c-5e74-4566-aa08-0afa07e226cc', 'b912845d-af2b-494d-b232-51a35ae3667c', NULL, NULL, NULL, '08b99ec7-f295-4fc1-914a-8b22db47726b', NULL, NULL, NULL),
('da30f442-0077-4c5b-92e2-803e5f0e2906', '45b2dd9a-d982-4326-8671-5a4850869553', NULL, NULL, NULL, NULL, NULL, NULL, 'dd7aa94d-6646-4f58-9552-8be95deeb6a9'),
('e311ce0d-90ff-4021-b149-512e6262562b', '27a9ca5a-3bde-42cc-a023-093ebc90d859', NULL, NULL, NULL, NULL, '3af27a3c-c105-44d3-b11a-90658e3336c3', NULL, NULL),
('e803a15e-a244-4f1a-aec1-07d33b210a2b', 'db576cb7-dffb-4e42-bf32-b9707f98fad3', NULL, NULL, NULL, 'a8d98067-82e4-474a-8427-c94439004f19', NULL, NULL, NULL),
('f14e7f65-ee84-4281-b2a5-d0b6cbc28cb6', '36c63be8-eb03-447d-be10-fca1151dbc93', NULL, NULL, 'ee18f9e5-ca9a-41fb-bfd7-2324f3fff16a', NULL, NULL, NULL, NULL),
('f156e5c7-2210-4227-ab3c-13f4ffae1ffc', '4231c960-8d2a-4fca-b149-6845c32dccda', NULL, 'a7f88339-ed50-4d14-a75f-755fa67014a8', NULL, NULL, NULL, NULL, NULL),
('f5086ef0-177b-4c5f-9684-f05681be8ffc', 'efcd8428-80d2-45e3-9562-337a20c9da15', NULL, NULL, NULL, NULL, NULL, NULL, 'd7dff637-aecf-4147-85e9-31454c6ea58d'),
('fa63f384-64d9-4aac-91b1-5514a50433e6', 'b4ffdb57-047a-4bab-8d72-baf5ef76a196', NULL, NULL, NULL, '7d7a2b05-04ef-4a50-ae52-1cb507341e7e', NULL, NULL, NULL),
('fb0de9cf-a73c-4dfb-b681-279715ec39e4', '712de550-0f88-46e3-b74a-66098e4283d1', NULL, NULL, NULL, '7d7a2b05-04ef-4a50-ae52-1cb507341e7e', NULL, NULL, NULL),
('fd58f1de-f1f0-43d2-a549-f5f460406623', '6d950f07-0c90-4814-831c-d30b06d1942c', NULL, NULL, NULL, NULL, NULL, NULL, 'd7dff637-aecf-4147-85e9-31454c6ea58d');

INSERT INTO "public"."StunEffect" ("id") VALUES
('13f2942d-a6aa-41d8-b7b1-3adba00717ff'),
('cc00930f-cc36-45a7-9637-a56f60ff67c4'),
('d9b9d92c-86ba-4efd-939a-586f8c930bf9'),
('f402e236-8fc9-4d08-853a-1db34a38ee9d');

INSERT INTO "public"."SummonEffect" ("id") VALUES
('58eea1bd-ff17-4a89-9748-a4218f32b0e7');

INSERT INTO "public"."TauntEffect" ("id") VALUES
('0e6a08a5-ccc1-4e6f-8345-32303f3a7c2b');

ALTER TABLE "public"."Action" ADD FOREIGN KEY ("npc_id") REFERENCES "public"."Npc"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."Action" ADD FOREIGN KEY ("quest_id") REFERENCES "public"."Quest"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."Action" ADD FOREIGN KEY ("region_name") REFERENCES "public"."Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."Action" ADD FOREIGN KEY ("leyline_id") REFERENCES "public"."Leyline"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."ActionCompleted" ADD FOREIGN KEY ("hero_id") REFERENCES "public"."Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."DamageEffectDetails" ADD FOREIGN KEY ("damageEffectId") REFERENCES "public"."DamageEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."DeckCard" ADD FOREIGN KEY ("cardId") REFERENCES "public"."Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."Follower" ADD FOREIGN KEY ("attributes_id") REFERENCES "public"."Attributes"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."FollowerTalent" ADD FOREIGN KEY ("talentId") REFERENCES "public"."Talent"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."FollowerTalent" ADD FOREIGN KEY ("followerId") REFERENCES "public"."Follower"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."Hero" ADD FOREIGN KEY ("attributes_id") REFERENCES "public"."Attributes"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."Hero" ADD FOREIGN KEY ("inventory_id") REFERENCES "public"."Inventory"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."Hero" ADD FOREIGN KEY ("base_stats_id") REFERENCES "public"."BaseStats"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."Hero" ADD FOREIGN KEY ("deckId") REFERENCES "public"."Deck"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."HeroActions" ADD FOREIGN KEY ("hero_id") REFERENCES "public"."Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroActions" ADD FOREIGN KEY ("action_id") REFERENCES "public"."Action"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroCard" ADD FOREIGN KEY ("cardId") REFERENCES "public"."Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroCard" ADD FOREIGN KEY ("heroId") REFERENCES "public"."Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroQuests" ADD FOREIGN KEY ("quest_id") REFERENCES "public"."Quest"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroQuests" ADD FOREIGN KEY ("hero_id") REFERENCES "public"."Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroRegion" ADD FOREIGN KEY ("region_name") REFERENCES "public"."Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroRegion" ADD FOREIGN KEY ("hero_id") REFERENCES "public"."Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroResource" ADD FOREIGN KEY ("hero_id") REFERENCES "public"."Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroResource" ADD FOREIGN KEY ("resource_type_id") REFERENCES "public"."ResourceType"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroTalent" ADD FOREIGN KEY ("talentId") REFERENCES "public"."Talent"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."HeroTalent" ADD FOREIGN KEY ("heroId") REFERENCES "public"."Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."Item" ADD FOREIGN KEY ("active_inventory_id") REFERENCES "public"."Inventory"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."Item" ADD FOREIGN KEY ("backpack_inventory_id") REFERENCES "public"."Inventory"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."Leyline" ADD FOREIGN KEY ("npcId") REFERENCES "public"."Npc"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."Leyline" ADD FOREIGN KEY ("RegionName") REFERENCES "public"."Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."MinionEffect" ADD FOREIGN KEY ("cardId") REFERENCES "public"."Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."MinionEffectEffect" ADD FOREIGN KEY ("pickupEffectId") REFERENCES "public"."PickupEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."MinionEffectEffect" ADD FOREIGN KEY ("minionEffectId") REFERENCES "public"."MinionEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."MinionEffectEffect" ADD FOREIGN KEY ("resilienceEffectId") REFERENCES "public"."ResilienceEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."MinionEffectEffect" ADD FOREIGN KEY ("poisonEffectId") REFERENCES "public"."PoisonEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."MinionEffectEffect" ADD FOREIGN KEY ("chargeEffectId") REFERENCES "public"."ChargeEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."MinionEffectEffect" ADD FOREIGN KEY ("summonEffectId") REFERENCES "public"."SummonEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."MinionEffectEffect" ADD FOREIGN KEY ("tauntEffectId") REFERENCES "public"."TauntEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."MinionEffectEffect" ADD FOREIGN KEY ("lifestealEffectId") REFERENCES "public"."LifestealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."Npc" ADD FOREIGN KEY ("deckId") REFERENCES "public"."Deck"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."NpcCard" ADD FOREIGN KEY ("npcId") REFERENCES "public"."Npc"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."NpcCard" ADD FOREIGN KEY ("cardId") REFERENCES "public"."Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."Quest" ADD FOREIGN KEY ("region_name") REFERENCES "public"."Region"("name") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."RetinueSlot" ADD FOREIGN KEY ("hero_id") REFERENCES "public"."Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."RetinueSlot" ADD FOREIGN KEY ("followerId") REFERENCES "public"."Follower"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."SpellEffect" ADD FOREIGN KEY ("cardId") REFERENCES "public"."Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."SpellEffectEffect" ADD FOREIGN KEY ("spellEffectId") REFERENCES "public"."SpellEffect"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
ALTER TABLE "public"."SpellEffectEffect" ADD FOREIGN KEY ("initiativeEffectId") REFERENCES "public"."InitiativeEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."SpellEffectEffect" ADD FOREIGN KEY ("resilienceEffectId") REFERENCES "public"."ResilienceEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."SpellEffectEffect" ADD FOREIGN KEY ("damageEffectId") REFERENCES "public"."DamageEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."SpellEffectEffect" ADD FOREIGN KEY ("armorEffectId") REFERENCES "public"."ArmorEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."SpellEffectEffect" ADD FOREIGN KEY ("poisonEffectId") REFERENCES "public"."PoisonEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."SpellEffectEffect" ADD FOREIGN KEY ("healEffectId") REFERENCES "public"."HealEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;
ALTER TABLE "public"."SpellEffectEffect" ADD FOREIGN KEY ("stunEffectId") REFERENCES "public"."StunEffect"("id") ON DELETE SET NULL ON UPDATE CASCADE;

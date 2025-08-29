-- Drop dependent tables first
DROP TABLE IF EXISTS "public"."HeroSpell" CASCADE;
DROP TABLE IF EXISTS "public"."Relic" CASCADE;
DROP TABLE IF EXISTS "public"."HeroResource" CASCADE;
DROP TABLE IF EXISTS "public"."HeroActions" CASCADE;
DROP TABLE IF EXISTS "public"."HeroQuests" CASCADE;
DROP TABLE IF EXISTS "public"."ActionCompleted" CASCADE;
DROP TABLE IF EXISTS "public"."HeroRegion" CASCADE;
DROP TABLE IF EXISTS "public"."HeroCard" CASCADE;
DROP TABLE IF EXISTS "public"."Deck" CASCADE;
DROP TABLE IF EXISTS "public"."Stamina" CASCADE;
DROP TABLE IF EXISTS "public"."Hero" CASCADE;
DROP TABLE IF EXISTS "public"."Account" CASCADE;
DROP TABLE IF EXISTS "public"."Card" CASCADE;
DROP TABLE IF EXISTS "public"."CardEffect" CASCADE;
DROP TABLE IF EXISTS "public"."Region" CASCADE;
DROP TABLE IF EXISTS "public"."ResourceType" CASCADE;
DROP TABLE IF EXISTS "public"."Spell" CASCADE;

-- Drop types
DROP TYPE IF EXISTS "public"."Class" CASCADE;
DROP TYPE IF EXISTS "public"."CardType" CASCADE;
DROP TYPE IF EXISTS "public"."TargetType" CASCADE;
DROP TYPE IF EXISTS "public"."StatType" CASCADE;
DROP TYPE IF EXISTS "public"."EffectType" CASCADE;
DROP TYPE IF EXISTS "public"."ResourceEnum" CASCADE;

-- Create types
CREATE TYPE "public"."Class" AS ENUM ('Fighter', 'Ranger', 'Wizard');
CREATE TYPE "public"."CardType" AS ENUM ('Attack', 'Defensive', 'Utility');
CREATE TYPE "public"."TargetType" AS ENUM ('Opponent', 'Itself');
CREATE TYPE "public"."StatType" AS ENUM ('HP', 'Strength', 'Dexterity', 'Intelligence', 'Armor');
CREATE TYPE "public"."EffectType" AS ENUM ('Damage', 'Heal', 'BuffStat', 'BuffDamage', 'DebuffStat', 'DebuffDamage', 'ManaGain', 'Draw', 'Armor', 'Silence', 'Poison', 'Initiative');
CREATE TYPE "public"."ResourceEnum" AS ENUM ('Aion', 'Valor', 'NexusOrb', 'StormShard');

-- Create Account table
CREATE TABLE "public"."Account" (
    "id" text NOT NULL,
    "supabase_user_id" text NOT NULL,
    "created_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

-- Create unique index for Account
CREATE UNIQUE INDEX "Account_supabase_user_id_key" ON public."Account" USING btree (supabase_user_id);

-- Insert Account records
INSERT INTO "public"."Account" ("id", "supabase_user_id", "created_at", "updated_at") VALUES
('b6ee7843-bbd9-4937-8272-5c633e03b880', 'some_user', '2024-09-25 10:28:29.268', '2024-09-25 10:28:29.268'),
('c081e311-abe2-49f6-a48d-26dc4dbd1f53', 'secondId', '2024-09-25 10:28:17.81', '2024-09-25 10:28:17.81'),
('f42dc222-0dd2-43c4-b581-c36ded16bac5', 'marko911', '2024-09-25 10:27:34.298', '2024-09-25 10:27:34.298');

-- Create Hero table
CREATE TABLE "public"."Hero" (
    "id" text NOT NULL,
    "class" "public"."Class" NOT NULL,
    "hp" int4 NOT NULL,
    "strength" int4 NOT NULL,
    "dexterity" int4 NOT NULL,
    "intelligence" int4 NOT NULL,
    "explore" int4 NOT NULL,
    "crafting" int4 NOT NULL,
    "accountId" text,
    "level" int4 NOT NULL,
    "armor" int4 NOT NULL,
    "name" text NOT NULL,
    CONSTRAINT "Hero_accountId_fkey" FOREIGN KEY ("accountId") REFERENCES "public"."Account"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

-- Insert Hero records
INSERT INTO "public"."Hero" ("id", "class", "hp", "strength", "dexterity", "intelligence", "explore", "crafting", "accountId", "level", "armor", "name") VALUES
('2a038e2f-4de8-4907-b48a-c5ce89f876e3', 'Fighter', 89, 5, 2, 1, 15, 20, 'f42dc222-0dd2-43c4-b581-c36ded16bac5', 1, 1, 'ElFal the Fierce'),
('90e64955-1208-4e8d-ab5e-04b3dbe86b6b', 'Wizard', 77, 2, 2, 0, 15, 20, 'c081e311-abe2-49f6-a48d-26dc4dbd1f53', 1, 1, 'VorZaDra'),
('a50ce823-b66c-4873-bf4e-1b38b45f8e5e', 'Wizard', 92, 4, 1, 2, 15, 20, 'b6ee7843-bbd9-4937-8272-5c633e03b880', 1, 1, 'FalHelRo');

CREATE TABLE "public"."Deck" (
    "id" text NOT NULL,
    "name" text NOT NULL DEFAULT 'my deck'::text,
    "heroId" text,
    "active" bool NOT NULL DEFAULT false,
    "npcId" text,
    CONSTRAINT "Deck_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "public"."Hero"("id") ON DELETE SET NULL ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Deck" ("id", "name", "heroId", "active", "npcId") VALUES
('2e872d1f-83bf-41c8-9e11-c6ec7785a110', 'my deck', '90e64955-1208-4e8d-ab5e-04b3dbe86b6b', 'f', NULL),
('a741ac20-7cb0-4cb1-a423-22a99113ebc1', 'my deck', '2a038e2f-4de8-4907-b48a-c5ce89f876e3', 'f', NULL),
('b0ec518a-4232-4ad2-9155-4bba2905c325', 'my deck', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', 't', NULL);

-- Table Definition
CREATE TABLE "public"."Card" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "class" "public"."Class" NOT NULL,
    "cost" int4 NOT NULL,
    "imgUrl" text NOT NULL,
    "tier" int4 NOT NULL,
    "cardType" "public"."CardType" NOT NULL,
    "zeal" int4 NOT NULL,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."Card" ("id", "name", "class", "cost", "imgUrl", "tier", "cardType", "zeal") VALUES
('01114635-0406-428f-886a-c0aa85522b58', 'Strengthen', 'Fighter', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 1, 'Utility', 1),
('0ba29bce-51d1-46ee-8fed-469aae5f9952', 'Stealth', 'Ranger', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 1, 'Utility', 1),
('13d49de3-0208-461d-945e-13c92ac0f48b', 'Arcane Intellect', 'Wizard', 1, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 1, 'Utility', 1),
('20053aa3-278d-4349-b696-67bd059f11ec', 'Precise Shot', 'Ranger', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 1, 'Attack', 1),
('2b0a24f6-be6f-4d33-92e7-a10c502a956b', 'Mana Surge', 'Wizard', 0, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 1, 'Utility', 1),
('2e6f87d1-2336-4d9c-98a6-9c31fb0772b6', 'Shield Wall', 'Fighter', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 1, 'Defensive', 1),
('420086bd-3b82-4409-af5a-c1bee18321b8', 'Berserker Rage', 'Fighter', 3, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 2, 'Utility', 2),
('4ccd1870-f8e0-4fb3-9b95-f5ba4b24150e', 'Arcane Explosion', 'Wizard', 4, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 2, 'Attack', 2),
('5bd8ef79-24a2-45e1-93ba-8247f076b325', 'Frost Nova', 'Wizard', 3, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 2, 'Attack', 2),
('647f462a-bcb3-4340-ba8c-19fd070dcb92', 'Fireball', 'Wizard', 2, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 1, 'Attack', 1),
('760ee21e-0eb4-46bb-8d03-f1add10f1ee7', 'Time Warp', 'Wizard', 5, 'https://imagedelivery.net/OnMBHZ1Vl1a2W_jlBO5uVw/468fd353-93c2-46bb-52d5-d0535fc11900/public', 3, 'Utility', 3),
('873e9d4c-8245-4f7b-bcf5-ceb040c12e34', 'Fortify', 'Fighter', 2, 'https://example.com/fortify.jpg', 1, 'Defensive', 1),
('94dcc3dd-84d4-45f0-a128-2f811008b0ae', 'Quick Shot', 'Ranger', 1, 'https://example.com/quick-shot.jpg', 1, 'Attack', 1),
('9b49c02d-f07d-4759-9865-ad6c1eee923e', 'Poison Dart', 'Ranger', 1, 'https://example.com/poison-dart.jpg', 1, 'Attack', 1),
('a06591da-5a0d-4ae1-9b23-a38200b4bf18', 'Healing Potion', 'Fighter', 2, 'https://example.com/healing-potion.jpg', 1, 'Utility', 1),
('a852796f-5118-422c-b2a2-cd861e00124b', 'Volley', 'Ranger', 3, 'https://example.com/volley.jpg', 2, 'Attack', 2),
('a8d9102b-84b2-4170-a779-22b647f64308', 'Polymorph', 'Wizard', 4, 'https://example.com/polymorph.jpg', 2, 'Utility', 2),
('d9a2506c-8401-4e88-801b-cff9df397b3b', 'Whirlwind', 'Fighter', 3, 'https://example.com/whirlwind.jpg', 2, 'Attack', 2),
('ebed16f0-d638-4957-9d94-2ac08401346b', 'Crushing Blow', 'Fighter', 3, 'https://example.com/crushing-blow.jpg', 2, 'Attack', 2),
('f92df2bd-6339-4dc4-a6e8-d60dba207ac1', 'Evasive Maneuver', 'Ranger', 1, 'https://example.com/evasive-maneuver.jpg', 1, 'Defensive', 1);

-- Table Definition
CREATE TABLE "public"."CardEffect" (
    "id" text NOT NULL,
    "value" int4 NOT NULL,
    "target" "public"."TargetType" NOT NULL,
    "duration" int4,
    "cardId" text NOT NULL,
    "percentageModifier" bool NOT NULL DEFAULT false,
    "statAffected" "public"."StatType",
    "effectType" "public"."EffectType" NOT NULL,
    CONSTRAINT "CardEffect_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "public"."Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."CardEffect" ("id", "value", "target", "duration", "cardId", "percentageModifier", "statAffected", "effectType") VALUES
('1dc63207-04ab-4578-bf2a-75d60f953760', 10, 'Itself', 2, '0ba29bce-51d1-46ee-8fed-469aae5f9952', 'f', 'Dexterity', 'BuffStat'),
('3d73bac4-4a75-4eca-b329-9bb05f7bc2d2', 3, 'Itself', NULL, '2b0a24f6-be6f-4d33-92e7-a10c502a956b', 'f', NULL, 'ManaGain'),
('3e642083-eb56-499c-b34b-063363890ae7', 1, 'Opponent', 2, 'a8d9102b-84b2-4170-a779-22b647f64308', 'f', NULL, 'Silence'),
('52b8d916-8c8b-45e7-a0ed-64170915ea79', 1, 'Itself', NULL, '0ba29bce-51d1-46ee-8fed-469aae5f9952', 'f', NULL, 'Draw'),
('62af4a26-e646-4d4e-b785-4279316cd2f8', 2, 'Opponent', 3, '9b49c02d-f07d-4759-9865-ad6c1eee923e', 'f', NULL, 'Poison'),
('6573cad5-610b-4be1-bbdb-1cc6eccf6fb6', 3, 'Opponent', 2, 'ebed16f0-d638-4957-9d94-2ac08401346b', 'f', 'Armor', 'DebuffStat'),
('66bcaa6c-3a34-4103-a29f-01867b08b51c', 5, 'Opponent', NULL, 'a852796f-5118-422c-b2a2-cd861e00124b', 'f', NULL, 'Damage'),
('750b8f3b-424d-4252-b898-bbc69969b173', 5, 'Itself', 3, '420086bd-3b82-4409-af5a-c1bee18321b8', 'f', 'Armor', 'DebuffStat'),
('75b76f7a-4159-449f-8090-e2b34ce0fd88', 2, 'Itself', NULL, '13d49de3-0208-461d-945e-13c92ac0f48b', 'f', NULL, 'ManaGain'),
('76c8245c-c2d5-4aab-8f27-f1f095c078bb', 2, 'Itself', 3, '760ee21e-0eb4-46bb-8d03-f1add10f1ee7', 'f', 'Intelligence', 'BuffStat'),
('7fb34ea5-459a-4b30-a1cb-cb81bfd55850', 5, 'Itself', NULL, '760ee21e-0eb4-46bb-8d03-f1add10f1ee7', 'f', NULL, 'ManaGain'),
('80549221-9a68-4e35-9e2c-b42106d77fef', 3, 'Itself', 3, '01114635-0406-428f-886a-c0aa85522b58', 'f', 'Strength', 'BuffStat'),
('857adc87-43b2-44ff-b1a2-d0c672a773a2', 18, 'Opponent', NULL, 'ebed16f0-d638-4957-9d94-2ac08401346b', 'f', NULL, 'Damage'),
('889f9c02-f7da-4978-aea8-39f43e778162', 2, 'Itself', NULL, '760ee21e-0eb4-46bb-8d03-f1add10f1ee7', 'f', NULL, 'Draw'),
('8de02f65-c127-414e-abe1-d0afa7ff59da', 5, 'Itself', 2, 'f92df2bd-6339-4dc4-a6e8-d60dba207ac1', 'f', 'Dexterity', 'BuffStat'),
('8fdbeb15-13f7-41a1-9755-5999c3f3e978', 10, 'Opponent', NULL, '20053aa3-278d-4349-b696-67bd059f11ec', 'f', NULL, 'Damage'),
('91c72830-d0d2-4a5e-943b-d18a1cffd7c6', 8, 'Itself', 3, '873e9d4c-8245-4f7b-bcf5-ceb040c12e34', 'f', 'Armor', 'BuffStat'),
('946c00fa-97a1-40c9-b554-034781169a41', 12, 'Opponent', NULL, '4ccd1870-f8e0-4fb3-9b95-f5ba4b24150e', 'f', NULL, 'Damage'),
('9c2ccc2f-4f40-409a-80cb-d7c0bf789050', 3, 'Opponent', NULL, '9b49c02d-f07d-4759-9865-ad6c1eee923e', 'f', NULL, 'Damage'),
('a6aa340f-e781-44ed-a2a4-2ceb3a2911fb', 3, 'Itself', 2, 'd9a2506c-8401-4e88-801b-cff9df397b3b', 'f', 'Armor', 'BuffStat'),
('a8947e61-3ce6-4aba-aa71-b98041d95d84', 50, 'Itself', 3, '420086bd-3b82-4409-af5a-c1bee18321b8', 't', NULL, 'BuffDamage'),
('ab535e47-f796-4b63-9ec0-cca10d8e8413', 30, 'Opponent', 2, '4ccd1870-f8e0-4fb3-9b95-f5ba4b24150e', 't', NULL, 'DebuffDamage'),
('b5400fb8-ce04-40ef-9885-772cc7f5524d', 2, 'Opponent', 2, '5bd8ef79-24a2-45e1-93ba-8247f076b325', 'f', 'Dexterity', 'DebuffStat'),
('b787ab60-8746-41b8-815a-7c59fd64254b', 20, 'Itself', 1, '20053aa3-278d-4349-b696-67bd059f11ec', 't', NULL, 'BuffDamage'),
('bf95d404-a1f5-4799-ab16-55231d76cc05', 10, 'Itself', NULL, '2e6f87d1-2336-4d9c-98a6-9c31fb0772b6', 'f', NULL, 'Armor'),
('c089e249-6822-42f9-ab37-c8b237af1aae', 12, 'Opponent', NULL, 'd9a2506c-8401-4e88-801b-cff9df397b3b', 'f', NULL, 'Damage'),
('c454a63c-19e7-4057-9c12-c998cd810aa2', 5, 'Opponent', NULL, 'a852796f-5118-422c-b2a2-cd861e00124b', 'f', NULL, 'Damage'),
('c9e6b0a4-db86-4a03-ad50-500b1bef705d', 8, 'Opponent', NULL, '94dcc3dd-84d4-45f0-a128-2f811008b0ae', 'f', NULL, 'Damage'),
('cb9286bb-6521-43c5-8125-c1aba2981c79', 5, 'Opponent', NULL, 'a852796f-5118-422c-b2a2-cd861e00124b', 'f', NULL, 'Damage'),
('e6ee07e9-438f-4fc6-9e7b-a80151fe62d8', 15, 'Opponent', NULL, '647f462a-bcb3-4340-ba8c-19fd070dcb92', 'f', NULL, 'Damage'),
('eadbf405-e1fe-4abf-a11e-3b4ef99e2f09', 70, 'Opponent', 2, 'a8d9102b-84b2-4170-a779-22b647f64308', 't', NULL, 'DebuffDamage'),
('eb8d210c-f7a7-4d7f-a652-d4f74e2556fa', 1, 'Itself', NULL, '13d49de3-0208-461d-945e-13c92ac0f48b', 'f', NULL, 'Draw'),
('f8c87f61-fa97-4025-9a69-83d67d1d2e80', 8, 'Opponent', NULL, '5bd8ef79-24a2-45e1-93ba-8247f076b325', 'f', NULL, 'Damage'),
('f8fe2ff1-a56f-44cc-8f96-4e1258186dbf', 15, 'Itself', NULL, 'a06591da-5a0d-4ae1-9b23-a38200b4bf18', 'f', NULL, 'Heal');

-- Table Definition
CREATE TABLE "public"."Region" (
    "name" text NOT NULL,
    "adjacent_regions" text[],
    PRIMARY KEY ("name")
);

INSERT INTO "public"."Region" ("name", "adjacent_regions") VALUES
('Buzna', '{Lindon}'),
('Dusane', '{Veladria}'),
('Emerlad', '{Veladria,Lindon,Yezer,Forest}'),
('Forest', '{Yezer,Emerlad}'),
('Lindon', '{Veladria,Lindon,Yezer,Buzna}'),
('Veladria', '{Dusane,Lindon,Emerlad}'),
('Yezer', '{Lindon,Emerlad,Forest}');

-- Table Definition
CREATE TABLE "public"."ResourceType" (
    "id" text NOT NULL,
    "type" "public"."ResourceEnum" NOT NULL,
    PRIMARY KEY ("id")
);

-- Indices
CREATE UNIQUE INDEX "ResourceType_type_key" ON public."ResourceType" USING btree (type);

INSERT INTO "public"."ResourceType" ("id", "type") VALUES
('105d5452-bfe0-44c6-893d-18cad6192eee', 'NexusOrb'),
('4b92d9a3-8d30-4895-babb-066ef85a3b4c', 'StormShard'),
('64bb06f5-04e4-42d9-97f0-28f31f89b370', 'Valor'),
('e1059285-3b37-4224-8bd9-5653cb8c3586', 'Aion');



-- Create Spell table
CREATE TABLE "public"."Spell" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    "level" int4 NOT NULL,
    "duration" int4,
    PRIMARY KEY ("id")
);

-- Insert Spell records
INSERT INTO "public"."Spell" ("id", "name", "level", "duration") VALUES
('1a2b3c4d-5e6f-7g8h-9i0j-1k2l3m4n5o6p', 'Fireball', 1, 1),
('2b3c4d5e-6f7g-8h9i-0j1k-2l3m4n5o6p7q', 'Ice Lance', 1, 1),
('3c4d5e6f-7g8h-9i0j-1k2l-3m4n5o6p7q8r', 'Healing Light', 2, 3),
('4d5e6f7g-8h9i-0j1k-2l3m-4n5o6p7q8r9s', 'Thunder Strike', 2, 1),
('5e6f7g8h-9i0j-1k2l-3m4n-5o6p7q8r9s0t', 'Nature''s Embrace', 3, 5);

-- Create HeroSpell table
CREATE TABLE "public"."HeroSpell" (
    "id" text NOT NULL,
    "slot" int4 NOT NULL,
    "heroId" text NOT NULL,
    "spellId" text NOT NULL,
    CONSTRAINT "HeroSpell_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "public"."Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "HeroSpell_spellId_fkey" FOREIGN KEY ("spellId") REFERENCES "public"."Spell"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

-- Insert HeroSpell records
INSERT INTO "public"."HeroSpell" ("id", "slot", "heroId", "spellId") VALUES
('a1b2c3d4-e5f6-g7h8-i9j0-k1l2m3n4o5p6', 1, '2a038e2f-4de8-4907-b48a-c5ce89f876e3', '4d5e6f7g-8h9i-0j1k-2l3m-4n5o6p7q8r9s'),
('b2c3d4e5-f6g7-h8i9-j0k1-l2m3n4o5p6q7', 2, '2a038e2f-4de8-4907-b48a-c5ce89f876e3', '3c4d5e6f-7g8h-9i0j-1k2l-3m4n5o6p7q8r'),
('c3d4e5f6-g7h8-i9j0-k1l2-m3n4o5p6q7r8', 1, '90e64955-1208-4e8d-ab5e-04b3dbe86b6b', '1a2b3c4d-5e6f-7g8h-9i0j-1k2l3m4n5o6p'),
('d4e5f6g7-h8i9-j0k1-l2m3-n4o5p6q7r8s9', 2, '90e64955-1208-4e8d-ab5e-04b3dbe86b6b', '2b3c4d5e-6f7g-8h9i-0j1k-2l3m4n5o6p7q'),
('e5f6g7h8-i9j0-k1l2-m3n4-o5p6q7r8s9t0', 3, '90e64955-1208-4e8d-ab5e-04b3dbe86b6b', '5e6f7g8h-9i0j-1k2l-3m4n-5o6p7q8r9s0t'),
('f6g7h8i9-j0k1-l2m3-n4o5-p6q7r8s9t0u1', 1, 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', '1a2b3c4d-5e6f-7g8h-9i0j-1k2l3m4n5o6p'),
('g7h8i9j0-k1l2-m3n4-o5p6-q7r8s9t0u1v2', 2, 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', '2b3c4d5e-6f7g-8h9i-0j1k-2l3m4n5o6p7q');
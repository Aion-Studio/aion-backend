-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-06-27 8:53:52.3460 AM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."TargetType";
CREATE TYPE "public"."TargetType" AS ENUM ('Opponent', 'Itself');
DROP TYPE IF EXISTS "public"."StatType";
CREATE TYPE "public"."StatType" AS ENUM ('HP', 'Strength', 'Dexterity', 'Intelligence', 'Armor');
DROP TYPE IF EXISTS "public"."EffectType";
CREATE TYPE "public"."EffectType" AS ENUM ('Damage', 'Heal', 'BuffStat', 'BuffDamage', 'DebuffStat', 'DebuffDamage', 'ManaGain', 'Draw', 'Armor', 'Silence', 'Poison', 'Initiative');

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

CREATE TABLE "public"."HeroCard" (
    "id" text NOT NULL,
    "heroId" text NOT NULL,
    "cardId" text NOT NULL,
    CONSTRAINT "HeroCard_heroId_fkey" FOREIGN KEY ("heroId") REFERENCES "public"."Hero"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "HeroCard_cardId_fkey" FOREIGN KEY ("cardId") REFERENCES "public"."Card"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    PRIMARY KEY ("id")
);

INSERT INTO "public"."HeroCard" ("id", "heroId", "cardId") VALUES
('30f5da50-e679-4d23-87da-af9ded3f39ed', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', 'a06591da-5a0d-4ae1-9b23-a38200b4bf18'),
('58c96aa4-7653-47e4-8f65-d964137d50e6', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', 'a852796f-5118-422c-b2a2-cd861e00124b'),
('5f583ff3-35b4-41d6-a9d6-0fbd978c591e', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', '13d49de3-0208-461d-945e-13c92ac0f48b'),
('691f4e1d-ac77-4433-9dfe-74560a485ad3', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', '94dcc3dd-84d4-45f0-a128-2f811008b0ae'),
('70543943-7b89-4c16-98e8-8f3683b920cc', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', 'd9a2506c-8401-4e88-801b-cff9df397b3b'),
('822f70c3-7ba6-406b-b9e2-8b74f3957ff4', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', 'ebed16f0-d638-4957-9d94-2ac08401346b'),
('870126c1-4f8f-4af0-833e-67147b959f55', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', '873e9d4c-8245-4f7b-bcf5-ceb040c12e34'),
('a5523e63-f137-4c55-add7-e602b3822706', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', '9b49c02d-f07d-4759-9865-ad6c1eee923e'),
('cf86d9d6-b14e-4081-b944-f1c654772802', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', 'a8d9102b-84b2-4170-a779-22b647f64308'),
('f218dc45-5516-4775-ba24-d76be2d8858b', 'a50ce823-b66c-4873-bf4e-1b38b45f8e5e', 'f92df2bd-6339-4dc4-a6e8-d60dba207ac1');

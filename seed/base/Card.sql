-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-06-27 8:53:43.5790 AM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

DROP TYPE IF EXISTS "public"."Class";
CREATE TYPE "public"."Class" AS ENUM ('Fighter', 'Ranger', 'Wizard');
DROP TYPE IF EXISTS "public"."CardType";
CREATE TYPE "public"."CardType" AS ENUM ('Attack', 'Defensive', 'Utility');

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

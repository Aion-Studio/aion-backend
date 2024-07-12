-- -------------------------------------------------------------
-- TablePlus 6.0.0(550)
--
-- https://tableplus.com/
--
-- Database: defaultdb
-- Generation Time: 2024-06-27 8:53:16.2400 AM
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. Do not use it as a backup.

-- Table Definition
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

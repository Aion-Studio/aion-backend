/*
  Warnings:

  - The values [Magic] on the enum `Rarity` will be removed. If these variants are still used in the database, this will fail.

*/
-- AlterEnum
BEGIN;
CREATE TYPE "Rarity_new" AS ENUM ('Common', 'Rare', 'Epic', 'Legendary');
ALTER TABLE "Card" ALTER COLUMN "rarity" TYPE "Rarity_new" USING ("rarity"::text::"Rarity_new");
ALTER TYPE "Rarity" RENAME TO "Rarity_old";
ALTER TYPE "Rarity_new" RENAME TO "Rarity";
DROP TYPE "Rarity_old";
COMMIT;

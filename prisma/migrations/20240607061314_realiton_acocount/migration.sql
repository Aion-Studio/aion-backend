/*
  Warnings:

  - Added the required column `account_id` to the `Hero` table. 
  - Will update existing rows where `account_id` is NULL to a default value.

*/
-- AlterTable
ALTER TABLE "Hero" ADD COLUMN     "account_id" TEXT; -- Add the column first

-- Update existing null `account_id` values with a placeholder value
UPDATE "Hero" SET "account_id" = 'tempid123' WHERE "account_id" IS NULL;

-- Make account_id NOT NULL
ALTER TABLE "Hero" ALTER COLUMN "account_id" SET NOT NULL;

-- AddForeignKey
ALTER TABLE "Hero" ADD CONSTRAINT "Hero_accountId_fkey" FOREIGN KEY ("account_id") REFERENCES "Account"("id") ON DELETE CASCADE;

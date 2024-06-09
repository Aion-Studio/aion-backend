-- DropForeignKey
ALTER TABLE "Hero" DROP CONSTRAINT "Hero_accountId_fkey";

-- AddForeignKey
ALTER TABLE "Hero" ADD CONSTRAINT "Hero_account_id_fkey" FOREIGN KEY ("account_id") REFERENCES "Account"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

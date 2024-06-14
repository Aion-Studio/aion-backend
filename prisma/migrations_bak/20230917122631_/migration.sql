/*
  Warnings:

  - You are about to drop the column `create_time` on the `ActionCompleted` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "ActionCompleted" DROP COLUMN "create_time",
ADD COLUMN     "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP;

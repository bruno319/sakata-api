ALTER TABLE `players`
ADD COLUMN `discord_id` BIGINT NOT NULL;

ALTER TABLE `players`
ADD UNIQUE (`discord_id`);
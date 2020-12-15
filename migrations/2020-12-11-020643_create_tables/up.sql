CREATE TABLE IF NOT EXISTS `base_cards` (
    `id` INT(11) UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `name` VARCHAR(40) NOT NULL,
    `overall_power` TINYINT UNSIGNED NOT NULL,
    `class` TINYINT NOT NULL,
    `domain` TINYINT NOT NULL,
    `mal_id` INT(11) NOT NULL
);

CREATE TABLE IF NOT EXISTS `players` (
    `id`INT(11) UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `discord_id` BIGINT NOT NULL UNIQUE,
    `nickname` VARCHAR(40) NOT NULL,
    `coins` SMALLINT  NOT NULL,
    `stardust` SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS `player_cards` (
    `id` VARCHAR(36) PRIMARY KEY,
    `base_card_id` INT(11) UNSIGNED NOT NULL,
    `player_id` INT(11) UNSIGNED NOT NULL,
    `rarity` TINYINT NOT NULL,
    `quantity` TINYINT UNSIGNED NOT NULL,
    FOREIGN KEY(`base_card_id`) REFERENCES `base_cards`(`id`),
    FOREIGN KEY(`player_id`) REFERENCES `players`(`id`)
);

CREATE TABLE IF NOT EXISTS `party` (
    `id` BIGINT AUTO_INCREMENT PRIMARY KEY,
    `power` SMALLINT UNSIGNED NOT NULL,
    `card_1` VARCHAR(36) NOT NULL,
    `card_2` VARCHAR(36) NOT NULL,
    `card_3` VARCHAR(36) NOT NULL,
    `card_4` VARCHAR(36) NOT NULL,
    `card_5` VARCHAR(36) NOT NULL,
    FOREIGN KEY(`id`) REFERENCES `players`(`discord_id`),
    FOREIGN KEY(`card_1`) REFERENCES `player_cards`(`id`),
    FOREIGN KEY(`card_2`) REFERENCES `player_cards`(`id`),
    FOREIGN KEY(`card_3`) REFERENCES `player_cards`(`id`),
    FOREIGN KEY(`card_4`) REFERENCES `player_cards`(`id`),
    FOREIGN KEY(`card_5`) REFERENCES `player_cards`(`id`)
);
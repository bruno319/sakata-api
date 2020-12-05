DROP TABLE IF EXISTS `base_cards`;

CREATE TABLE `base_cards` (
    `id` INT(11) AUTO_INCREMENT PRIMARY KEY,
    `name` VARCHAR(40) NOT NULL,
    `overall_power` TINYINT NOT NULL,
    `class` TINYINT NOT NULL,
    `genre` TINYINT NOT NULL,
    `mal_id` INT NOT NULL
);

CREATE TABLE `players` (
    `id` INT(11) AUTO_INCREMENT PRIMARY KEY,
    `nickname` VARCHAR(40) NOT NULL,
    `coins` SMALLINT NOT NULL,
    `stardust` SMALLINT NOT NULL
);

CREATE TABLE `player_cards` (
    `id` INT(11) AUTO_INCREMENT PRIMARY KEY,
    `base_card_id` INT(11) NOT NULL,
    `player_id` INT(11) NOT NULL,
    `rarity` TINYINT NOT NULL,
    `quantity` TINYINT NOT NULL,
    FOREIGN KEY(`base_card_id`) REFERENCES `base_cards`(`id`),
    FOREIGN KEY(`player_id`) REFERENCES `players`(`id`)
);
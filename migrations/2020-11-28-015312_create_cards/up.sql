CREATE TABLE cards (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(40) NOT NULL,
    overall_power TINYINT NOT NULL,
    class TINYINT NOT NULL,
    genre TINYINT NOT NULL,
    rarity TINYINT NOT NULL,
    image VARCHAR(255) NOT NULL
);
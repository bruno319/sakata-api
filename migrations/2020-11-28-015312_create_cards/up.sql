CREATE TABLE base_cards (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(40) NOT NULL,
    overall_power TINYINT NOT NULL,
    class TINYINT NOT NULL,
    genre TINYINT NOT NULL,
    mal_id INT NOT NULL
);
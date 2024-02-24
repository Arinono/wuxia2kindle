-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    discord_id VARCHAR(255) DEFAULT NULL UNIQUE,
    avatar VARCHAR(255) DEFAULT NULL
);

INSERT INTO users (username, discord_id) VALUES ('arinono', '390570915869753355');

-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,

    name VARCHAR(32) NOT NULL UNIQUE,

    qq BIGINT NOT NULL UNIQUE
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
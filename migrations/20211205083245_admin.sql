-- Add migration script here
CREATE TABLE IF NOT EXISTS admins (
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,

    `name` VARCHAR(32) NOT NULL UNIQUE,

    `password` VARCHAR(64) NOT NULL
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
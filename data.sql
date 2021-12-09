-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,

    `name` VARCHAR(32) NOT NULL UNIQUE,

    `qq` BIGINT NOT NULL UNIQUE,
    `password` VARCHAR(64) NOT NULL
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
-- next table 
-- Add migration script here
CREATE TABLE IF NOT EXISTS illustrators (
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,

    `name` VARCHAR(32) NOT NULL UNIQUE,
    `home` VARCHAR(256) NOT NULL UNIQUE

)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS illustrator_wants(
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,

    `uid` BIGINT NOT NULL,
    `iid` BIGINT NOT NULL,

    FOREIGN KEY (uid) REFERENCES users(id),
    FOREIGN KEY (iid) REFERENCES illustrators(id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
-- next table 
-- Add migration script here
CREATE TABLE IF NOT EXISTS illustrator_acts(
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,

    `iid` BIGINT NOT NULL,

    `is_suit` BOOL NOT NULL DEFAULT false,
    `pic` VARCHAR(256) NOT NULL,

    FOREIGN KEY (iid) REFERENCES illustrators(id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
-- next table 
-- Add migration script here
CREATE TABLE IF NOT EXISTS admins (
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,

    `name` VARCHAR(32) NOT NULL UNIQUE,

    `password` VARCHAR(64) NOT NULL
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
-- next table 
-- Add migration script here
CREATE TABLE IF NOT EXISTS invites (
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,

    `code` VARCHAR(36) NOT NULL UNIQUE

)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
-- next table 

-- Add migration script here
CREATE TABLE IF NOT EXISTS file_stores(
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,
    `uid` BIGINT NOT NULL,
    `is_suit` BOOL NOT NULL DEFAULT false,
    `file` VARCHAR(256) NOT NULL,

    FOREIGN KEY (`uid`) REFERENCES users(id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
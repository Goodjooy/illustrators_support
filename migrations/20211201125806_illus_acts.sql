-- Add migration script here
CREATE TABLE IF NOT EXISTS illustrator_acts(
    `id` BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,

    `iid` BIGINT NOT NULL,
    `fid`  BIGINT NOT NULL UNIQUE,

    FOREIGN KEY (`iid`) REFERENCES illustrators(id),
    FOREIGN KEY (`fid`) REFERENCES file_stores(id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
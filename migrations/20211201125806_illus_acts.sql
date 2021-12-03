-- Add migration script here
CREATE TABLE IF NOT EXISTS illustrator_acts(
    id BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE,

    iid BIGINT NOT NULL,

    pic VARCHAR(256) NOT NULL,

    FOREIGN KEY (iid) REFERENCES illustrators(id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
-- Add migration script here

CREATE TABLE IF NOT EXISTS update_record(
    -- 记录主键
    `id` BIGINT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    -- 改变时间
    `change_time` DATETIME NOT NULL,

    -- 修改的表名称
    `table_name` CHAR(64) NOT NULL,
    -- 对应元素主键id
    `change_id` BIGINT NOT NULL
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


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


CREATE TRIGGER  after_illustrator_create
AFTER INSERT
ON illustrators FOR EACH ROW
BEGIN
    INSERT INTO update_record(`change_time`,`table_name`,`change_id`)
    VALUES(NOW(),'illustrators',NEW.id);
END;



CREATE TRIGGER after_illustrator_wanter
AFTER INSERT
ON illustrator_wants FOR EACH ROW
BEGIN
    INSERT INTO update_record(`change_time`,`table_name`,`change_id`)
    VALUES(NOW(),'illustrator_wants',NEW.id);
END;



CREATE TRIGGER after_illustrator_arts
AFTER INSERT
ON illustrator_acts FOR EACH ROW
BEGIN
    INSERT INTO update_record(`change_time`,`table_name`,`change_id`)
    VALUES(NOW(),'illustrator_acts',NEW.id);
END;

CREATE TRIGGER after_storage_file
AFTER INSERT
ON file_stores FOR EACH ROW
BEGIN
    INSERT INTO update_record(`change_time`,`table_name`,`change_id`)
    VALUES(NOW(),'file_stores',NEW.id);
END;

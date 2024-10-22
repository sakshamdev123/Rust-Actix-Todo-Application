CREATE SCHEMA `test_db`;

CREATE TABLE `test_db`.`todos` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `title` VARCHAR(256) NOT NULL,
    `description` VARCHAR(512) NULL,
    -- status can be "New" or "Completed"
    `status` VARCHAR(16) NOT NULL DEFAULT "New",
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
);
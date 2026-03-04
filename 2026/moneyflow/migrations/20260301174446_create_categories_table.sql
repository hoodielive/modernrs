CREATE TABLE `categories`(
  `id` BIGINT unsigned NOT NULL auto_increment,
  `user_id` BIGINT unsigned NOT NULL,
  `name` VARCHAR(255) NOT NULL,
  `description` TEXT,
  `balance` BIGINT unsigned NOT NULL DEFAULT 0,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY(`id`),
  FOREIGN KEY(`user_id`) REFERENCES `users`(`id`)
ON DELETE cascade
);

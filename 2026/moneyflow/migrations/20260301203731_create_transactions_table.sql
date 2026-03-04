CREATE TABLE `transactions`(
  `id` BIGINT unsigned NOT NULL auto_increment,
  `user_id` BIGINT unsigned NOT NULL,
  `category_id` BIGINT unsigned NOT NULL,
  `type` VARCHAR(255) NOT NULL,
  `amount` BIGINT unsigned NOT NULL DEFAULT 0,
  `memo` VARCHAR(255) NOT NULL,
  `description` TEXT,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY(`id`),
  FOREIGN KEY(`user_id`) REFERENCES `users`(`id`)
ON DELETE cascade, 
FOREIGN KEY(`category_id`) REFERENCES `categories`(`id`)
ON DELETE cascade
);

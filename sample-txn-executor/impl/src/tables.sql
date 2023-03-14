CREATE TABLE Tasks
(
  subject						TEXT UNIQUE,
  creator						TEXT,
  worker						TEXT NULL,
  status						TEXT,
	price 						TEXT,
	required_deposit	TEXT
);
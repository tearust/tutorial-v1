CREATE TABLE Tasks
(
  subject						TEXT UNIQUE,
  creator						TEXT,
  worker						TEXT NULL,
  status						TEXT,
	price 						TEXT,
	required_deposit	TEXT
);
CREATE INDEX idx_subject ON Tasks (subject);

CREATE TABLE TaskExecution
(
  subject TEXT,
  worker  TEXT,
  price   TEXT
);
CREATE INDEX idx_subject ON TaskExecution (subject);
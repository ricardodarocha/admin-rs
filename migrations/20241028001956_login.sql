CREATE TABLE IF NOT EXISTS
login(login VARCHAR NOT NULL PRIMARY KEY, password VARCHAR NOT NULL, level VARCHAR NOT NULL);

INSERT OR IGNORE INTO login VALUES ('ADMIN', 'ADMIN', 'ADMIN');
INSERT OR IGNORE INTO login VALUES ('NICK', '128c5a', 'USER');
CREATE TABLE IF NOT EXISTS
usuarios(
    login VARCHAR NOT NULL PRIMARY KEY,
    nome VARCHAR NOT NULL, 
    senha VARCHAR NOT NULL, 
    nivel VARCHAR NOT NULL);

--TRÊS NIVES DE USUARIO, Admin, Supervisor e User
INSERT OR IGNORE INTO usuarios VALUES ('ADMIN', 'Administrador', 'ADMIN', 'ADMIN');
INSERT OR IGNORE INTO usuarios VALUES ('SUPER', 'Supervisor', 'SUPER', 'SUPER');
INSERT OR IGNORE INTO usuarios VALUES ('OPERA', 'Operador', '128c5a', 'USER');
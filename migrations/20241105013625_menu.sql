-- Add migration script here
CREATE TABLE IF NOT EXISTS
    menus (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        titulo VARCHAR NOT NULL,
        icone VARCHAR NOT NULL,
        link VARCHAR NOT NULL,
        tipo_usuario VARCHAR NOT NULL
);



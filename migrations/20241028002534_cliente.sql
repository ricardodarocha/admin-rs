CREATE TABLE IF NOT EXISTS
cliente(
    id  VARCHAR NOT NULL PRIMARY KEY, 
    nome VARCHAR NOT NULL, 
    cidade VARCHAR NOT NULL,
    avatar VARCHAR NOT NULL DEFAULT 'https://img.freepik.com/vetores-premium/avatar-icon0002_750950-43.jpg'
);

INSERT INTO cliente (id, nome, cidade) VALUES ('00008756486', 'John Doe', 'London');
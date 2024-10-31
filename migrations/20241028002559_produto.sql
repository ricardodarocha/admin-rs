CREATE TABLE IF NOT EXISTS
produto(
    id VARCHAR NOT NULL PRIMARY KEY,
    descricao VARCHAR NOT NULL,
    preco NUMERIC(16,2) NOT NULL DEFAULT 1.0,
    avatar VARCHAR NOT NULL DEFAULT 'https://images.vexels.com/content/146554/preview/square-white-cardboard-box-e007dd.png'
);

DELETE FROM produto;

INSERT INTO produto (id, descricao, preco, avatar) VALUES ('397dff', 'BISCOITO', 3.5, 'https://cached.imagescaler.hbpl.co.uk/resize/scaleWidth/743/cached.offlinehbpl.hbpl.co.uk/news/OMC/Permutivethumbnail.jpg');
INSERT INTO produto (id, descricao, preco, avatar) VALUES ('2f596c', 'REFRESCO', 6.0, 'https://atlas-content-cdn.pixelsquid.com/stock-images/orange-juice-n1P5A31-600.jpg');
INSERT INTO produto (id, descricao, preco, avatar) VALUES ('60128f', 'CHÁ', 1.2, 'https://i.pinimg.com/originals/ed/e8/16/ede816032ce12e000d34ece106a221da.png');
INSERT INTO produto (id, descricao, preco, avatar) VALUES ('d48552', 'CAFÉ', 1.2, 'https://img.freepik.com/vetores-gratis/xicara-realista-de-cafe-preto-na-ilustracao-vetorial-de-pires_1284-66002.jpg');
INSERT INTO produto (id, descricao, preco, avatar) VALUES ('d48553', 'LEITE', 2.0, 'https://w7.pngwing.com/pngs/600/735/png-transparent-coffee-milk-milk-bottle-milk.png');
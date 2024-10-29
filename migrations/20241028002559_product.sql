CREATE TABLE IF NOT EXISTS
product(
    id VARCHAR NOT NULL PRIMARY KEY,
    description VARCHAR NOT NULL,
    price NUMERIC(16,2) NOT NULL DEFAULT 1.0,
    avatar VARCHAR NOT NULL DEFAULT 'https://images.vexels.com/content/146554/preview/square-white-cardboard-box-e007dd.png'
);

DELETE FROM product;

INSERT INTO product (id, description, price, avatar) VALUES ('397dff', 'COOKIE', 3.5, 'https://cached.imagescaler.hbpl.co.uk/resize/scaleWidth/743/cached.offlinehbpl.hbpl.co.uk/news/OMC/Permutivethumbnail.jpg');
INSERT INTO product (id, description, price, avatar) VALUES ('2f596c', 'JUICE', 6.0, 'https://atlas-content-cdn.pixelsquid.com/stock-images/orange-juice-n1P5A31-600.jpg');
INSERT INTO product (id, description, price, avatar) VALUES ('60128f', 'TEA', 1.2, 'https://i.pinimg.com/originals/ed/e8/16/ede816032ce12e000d34ece106a221da.png');
INSERT INTO product (id, description, price, avatar) VALUES ('d48552', 'COFFE', 1.2, 'https://img.freepik.com/vetores-gratis/xicara-realista-de-cafe-preto-na-ilustracao-vetorial-de-pires_1284-66002.jpg');
INSERT INTO product (id, description, price, avatar) VALUES ('d48553', 'MILK', 2.0, 'https://w7.pngwing.com/pngs/600/735/png-transparent-coffee-milk-milk-bottle-milk.png');
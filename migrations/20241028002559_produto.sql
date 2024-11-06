CREATE TABLE IF NOT EXISTS
produto(
    id VARCHAR NOT NULL PRIMARY KEY,
    descricao VARCHAR NOT NULL,
    preco NUMERIC(16,2) NOT NULL DEFAULT 1.0,
    nome VARCHAR NOT NULL,
    avatar VARCHAR NOT NULL DEFAULT 'https://images.vexels.com/content/146554/preview/square-white-cardboard-box-e007dd.png'
);

INSERT INTO produto
(id, nome, descricao, preco, avatar) 
VALUES (
        'P006',
        'X-Burguer',
        'Pão, hambúrguer bovino, queijo, alface, tomate e maionese especial',
        1200,
        'https://picsum.photos/150?random=1'
    ),
    (
        'P007',
        'X-Salada',
        'Pão, hambúrguer bovino, queijo, alface, tomate, cebola e maionese caseira',
        1500,
        'https://picsum.photos/150?random=2'
    ),
    (
        'P008',
        'X-Bacon',
        'Pão, hambúrguer bovino, queijo, bacon crocante e molho barbecue',
        1800,
        'https://picsum.photos/150?random=3'
    ),
    (
        'P009',
        'X-Egg',
        'Pão, hambúrguer bovino, queijo, ovo frito, alface, tomate e maionese',
        1700,
        'https://picsum.photos/150?random=4'
    ),
    (
        'P101',
        'X-Frango',
        'Pão, filé de frango grelhado, queijo, alface, tomate e molho de iogurte',
        1600,
        'https://picsum.photos/150?random=5'
    ),
    (
        'P111',
        'X-Calabresa',
        'Pão, calabresa acebolada, queijo, tomate e maionese de alho',
        1800,
        'https://picsum.photos/150?random=6'
    ),
    (
        'P121',
        'X-Tudo',
        'Pão, hambúrguer bovino, queijo, ovo, bacon, calabresa, alface, tomate e molho especial',
        2200,
        'https://picsum.photos/150?random=7'
    ),
    (
        'P131',
        'Cheeseburger',
        'Pão, hambúrguer bovino e queijo cheddar derretido',
        1000,
        'https://picsum.photos/150?random=8'
    ),
    (
        'P141',
        'X-Duplo',
        'Pão, dois hambúrgueres bovinos, queijo, alface, tomate e molho secreto',
        2000,
        'https://picsum.photos/150?random=9'
    ),
    (
        'P151',
        'X-Veggie',
        'Pão integral, hambúrguer de grão-de-bico, alface, tomate, cenoura ralada e maionese vegana',
        1900,
        'https://picsum.photos/150?random=10'
    ),
    (
        'P161',
        'Hot Dog',
        'Pão de cachorro-quente, salsicha, ketchup, mostarda e batata palha',
        900,
        'https://picsum.photos/150?random=11'
    ),
    (
        'P171',
        'Hot Dog Especial',
        'Pão de cachorro-quente, duas salsichas, queijo, bacon e molho cheddar',
        1500,
        'https://picsum.photos/150?random=12'
    ),
    (
        'P181',
        'Sanduíche Natural',
        'Pão integral, peito de peru, queijo branco, alface, tomate e molho de ervas',
        1100,
        'https://picsum.photos/150?random=13'
    ),
    (
        'P191',
        'Torrada Mista',
        'Pão de forma, presunto, queijo e orégano, tostado na chapa',
        800,
        'https://picsum.photos/150?random=14'
    ),
    (
        'P202',
        'Wrap de Frango',
        'Tortilla recheada com frango grelhado, alface, tomate e molho ranch',
        1300,
        'https://picsum.photos/150?random=15'
    ),
    (
        'P212',
        'Wrap Vegano',
        'Tortilla recheada com tofu, legumes grelhados e molho tahine',
        1400,
        'https://picsum.photos/150?random=16'
    ),
    (
        'P222',
        'Hambúrguer Gourmet',
        'Pão brioche, hambúrguer de picanha, queijo brie, rúcula e cebola caramelizada',
        2500,
        'https://picsum.photos/150?random=17'
    ),
    (
        'P232',
        'Cachorro-Quente Gourmet',
        'Pão artesanal, salsicha artesanal, molho cheddar e cebola crispy',
        1700,
        'https://picsum.photos/150?random=18'
    ),
    (
        'P242',
        'Mini Hambúrguer',
        'Pão, mini hambúrguer bovino, queijo e ketchup',
        700,
        'https://picsum.photos/150?random=19'
    ),
    (
        'P252',
        'Sanduíche de Mortadela',
        'Pão francês, mortadela fatiada e queijo derretido',
        1200,
        'https://picsum.photos/150?random=20'
    ),
    (
        'P262',
        'Pão de Alho',
        'Pão recheado com manteiga de alho e queijo parmesão gratinado',
        800,
        'https://picsum.photos/150?random=21'
    ),
    (
        'P272',
        'Croissant de Presunto e Queijo',
        'Croissant recheado com presunto e queijo, assado na hora',
        1500,
        'https://picsum.photos/150?random=22'
    ),
    (
        'P282',
        'Pão na Chapa',
        'Pão francês crocante com manteiga derretida na chapa',
        500,
        'https://picsum.photos/150?random=23'
    ),
    (
        'P292',
        'Bauru',
        'Pão francês, rosbife, queijo derretido, tomate e orégano',
        1300,
        'https://picsum.photos/150?random=24'
    ),
    (
        'P303',
        'Tapioca de Frango',
        'Tapioca recheada com frango desfiado, queijo coalho e molho de pimenta',
        1200,
        'https://picsum.photos/150?random=25'
    ),
    (
        'P313',
        'Tapioca de Queijo',
        'Tapioca recheada com queijo coalho e orégano',
        1000,
        'https://picsum.photos/150?random=26'
    ),
    (
        'P323',
        'Panini Italiano',
        'Panini com salame, queijo provolone, rúcula e tomate seco',
        1600,
        'https://picsum.photos/150?random=27'
    ),
    (
        'P333',
        'Bagel de Salmão',
        'Bagel recheado com salmão defumado, cream cheese e cebolinha',
        2000,
        'https://picsum.photos/150?random=28'
    ),
    (
        'P343',
        'Queijo Quente',
        'Pão de forma com queijo derretido e crocante na chapa',
        700,
        'https://picsum.photos/150?random=29'
    ),
    (
        'P353',
        'Sanduíche de Pernil',
        'Pão francês, pernil desfiado, molho de cebola e pimentão',
        1500,
        'https://picsum.photos/150?random=30'
    );
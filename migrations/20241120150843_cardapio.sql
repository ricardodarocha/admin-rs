-- Add migration script here
alter table produto
add grupo varchar not null default 'PRODUTO';
INSERT INTO produto (id, descricao, grupo, nome, preco)
VALUES (
        "P201",
        "Molho de tomate, mussarela, manjericão fresco, azeite de oliva",
        "PIZZA",
        "Pizza Margherita",
        38
    ),
    (
        "PZ002",
        "Molho de tomate, mussarela, pepperoni",
        "PIZZA",
        "Pizza Pepperoni",
        40
    ),
    (
        "P203",
        "Molho de tomate, mussarela, parmesão, gorgonzola, provolone",
        "PIZZA",
        "Pizza Quatro Queijos",
        45
    ),
    (
        "P204",
        "Molho de tomate, pimentões, cebola, cogumelos, azeitonas, milho",
        "PIZZA",
        "Pizza Vegetariana",
        42
    ),
    (
        "P205",
        "Molho de tomate, mussarela, frango desfiado, catupiry",
        "PIZZA",
        "Pizza Frango com Catupiry",
        43
    ),
    ("B01", "", "BEBIDA", "Suco de Laranja", 8),
    ("B02", "", "BEBIDA", "Suco de Morango", 10),
    ("B03", "", "BEBIDA", "Refrigerante Lata", 7),
    (
        "S31",
        "",
        "SOBREMESA",
        "Pudim de Leite Condensado",
        12
    ),
    (
        "S32",
        "",
        "SOBREMESA",
        "Brownie com Sorvete",
        18
    ),
    ("S33", "", "SOBREMESA", "Torta de Limão", 15),
    ("S34", "", "SOBREMESA", "Petit Gateau", 20);
create table if not exists cardapio(
    id varchar not null default 'PRINCIPAL',
    produto varchar not null references produto(id),
    opcao varchar not null default 'PRINCIPAL',
    descricao varchar,
    disponivel integer not null default 9999,
    preco float
);

--insere todas as pizzas no cardápio
INSERT INTO cardapio (produto, opcao, descricao, preco)
SELECT produto.id AS produto,
    'FAMILIA' AS opcao,
    'Família' AS descricao,
    produto.preco * 1.5 AS preco
FROM produto
where grupo = 'PIZZA';
INSERT INTO cardapio (produto, opcao, descricao, preco)
SELECT produto.id AS produto,
    'MEDIA' AS opcao,
    'Média' AS descricao,
    produto.preco * 1.2 AS preco
FROM produto
where grupo = 'PIZZA';
INSERT INTO cardapio (produto, opcao, descricao, preco)
SELECT produto.id AS produto,
    'PEQUENA' AS opcao,
    'Pequena' AS descricao,
    produto.preco * 1 AS preco
FROM produto
where grupo = 'PIZZA';

--INSERE AS BEBIDAS NO CARDÁPIO
INSERT INTO cardapio (produto, opcao, descricao, preco)
SELECT produto.id AS produto,
    'PADRAO' AS opcao,
    'Padrão' AS descricao,
    produto.preco * 1 AS preco
FROM produto
where grupo = 'BEBIDA';

--INSERE AS SOBREMESAS NO CARDÁPIO
INSERT INTO cardapio (produto, opcao, descricao, preco)
SELECT produto.id AS produto,
    'PADRAO' AS opcao,
    'Padrão' AS descricao,
    produto.preco * 1 AS preco
FROM produto
where grupo = 'SOBREMESA';
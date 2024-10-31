CREATE TABLE IF NOT EXISTS
item (
    num_pedido INTEGER NOT NULL REFERENCES pedido(NUM),
    produto VARCHAR NOT NULL REFERENCES produto(ID),
    quant Float NOT NULL DEFAULT 1.0
);

INSERT OR IGNORE INTO item (num_pedido, produto, quant) VALUES (
    (SELECT num from pedido limit 1), 
    (SELECT id FROM produto WHERE descricao = 'BISCOITO'), 8.0 );
INSERT OR IGNORE INTO item (num_pedido, produto, quant) VALUES (
    (SELECT num from pedido limit 1), 
    (SELECT id FROM produto WHERE descricao = 'LEITE'), 3.0 );
INSERT OR IGNORE INTO item (num_pedido, produto, quant) VALUES (
    (SELECT num from pedido limit 1), 
    (SELECT id FROM produto WHERE descricao = 'REFRESCO'), 6.0 );
INSERT OR IGNORE INTO item (num_pedido, produto, quant) VALUES (
    (SELECT num from pedido limit 1), 
    (SELECT id FROM produto WHERE descricao = 'CAFÉ'), 5.0 );

-- Atualiza o valor
UPDATE pedido
SET valor = (
    SELECT SUM(i.quant * p.preco)
    FROM item i
    JOIN produto p ON i.produto = p.id
    WHERE i.num_pedido = pedido.num
);
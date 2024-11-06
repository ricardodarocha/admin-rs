CREATE TABLE IF NOT EXISTS item (
    num_pedido INTEGER NOT NULL REFERENCES pedido(NUM),
    produto VARCHAR NOT NULL REFERENCES produto(ID),
    quant Float NOT NULL DEFAULT 1.0
);
INSERT
    OR IGNORE INTO item (num_pedido, produto, quant)
select num,
    id,
    0.75 * abs(random() % 80) + 21
from pedido,
    (
        SELECT id
        FROM produto
        ORDER BY RANDOM()
        LIMIT 5
    )
order by num;
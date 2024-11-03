--Insere dados de teste para a tabela de itens do pedido

INSERT INTO item (num_pedido, produto, quant)
SELECT num,
    '397dff',
    valor
FROM pedido
WHERE cliente <> '00008756486';
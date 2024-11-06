DROP VIEW IF EXISTS pedido_view;
CREATE VIEW pedido_view AS
select id,
    nome,
    sum(quant),
    avg(quant * preco)
FROM item
    join produto on produto.id = item.produto
group by id,
    nome;

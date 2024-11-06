
-- venda média por produto

select id, nome, sum(quant), avg(quant * preco) FROM
item join produto on produto.id = item.produto
group by id, nome;
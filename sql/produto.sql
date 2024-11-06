

select id, nome, quant, avg(total) FROM
item join produto on produto.id = item.produto;
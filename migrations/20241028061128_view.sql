DROP VIEW IF EXISTS pedido_view;

CREATE VIEW pedido_view AS
    SELECT o.*,
           c.id name,
           c.cidade,
           i.*,
           p.descricao,
           p.preco,
           p.preco * i.quant AS total
      FROM pedido o
           JOIN cliente c ON o.cliente = c.ID
           JOIN cidade R ON R.nome = c.cidade
           JOIN ITEM i ON i.num_pedido = o.num
           JOIN produto P ON P.ID = i.produto
           ;
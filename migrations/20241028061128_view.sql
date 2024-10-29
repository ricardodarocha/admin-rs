DROP VIEW order_view;

CREATE VIEW order_view AS
    SELECT o.*,
           c.id name,
           c.city,
           i.*,
           p.description,
           p.price,
           p.price * i.quant AS total
      FROM orders o
           JOIN contact c ON o.customer = c.ID
           JOIN city R ON R.name = c.city
           JOIN ITEM i ON i.num_order = o.num
           JOIN product P ON P.ID = i.product
           ;
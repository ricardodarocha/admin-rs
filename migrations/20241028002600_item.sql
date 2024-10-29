CREATE TABLE IF NOT EXISTS
item (
    num_order INTEGER NOT NULL REFERENCES orders(NUM),
    product VARCHAR NOT NULL REFERENCES product(ID),
    quant Float NOT NULL DEFAULT 1.0
);

INSERT OR IGNORE INTO item (num_order, product, quant) VALUES (
    (SELECT num from orders limit 1), 
    (SELECT id FROM product WHERE description = 'COOKIE'), 8.0 );
INSERT OR IGNORE INTO item (num_order, product, quant) VALUES (
    (SELECT num from orders limit 1), 
    (SELECT id FROM product WHERE description = 'MILK'), 3.0 );
INSERT OR IGNORE INTO item (num_order, product, quant) VALUES (
    (SELECT num from orders limit 1), 
    (SELECT id FROM product WHERE description = 'JUICE'), 6.0 );
INSERT OR IGNORE INTO item (num_order, product, quant) VALUES (
    (SELECT num from orders limit 1), 
    (SELECT id FROM product WHERE description = 'COFFE'), 5.0 );

-- Update amount
UPDATE orders
SET amount = (
    SELECT SUM(i.quant * p.price)
    FROM item i
    JOIN product p ON i.product = p.id
    WHERE i.num_order = orders.num
);
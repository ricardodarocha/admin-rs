CREATE TABLE IF NOT EXISTS
orders (
    num INTEGER PRIMARY KEY AUTOINCREMENT,
    date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    customer VARCHAR NOT NULL REFERENCES contact(ID),
    amount Float,
    status VARHCAR NOT NULL DEFAULT 'new' CHECK(status IN ('new', 'preparing', 'ready') )
);

INSERT INTO orders (customer, amount) VALUES ('00008756486', 0.0 );
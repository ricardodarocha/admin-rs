CREATE TABLE IF NOT EXISTS
pedido (
    num INTEGER PRIMARY KEY AUTOINCREMENT,
    data TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    cliente VARCHAR NOT NULL REFERENCES cliente(ID),
    valor Float,
    status VARHCAR NOT NULL DEFAULT 'novo' CHECK(status IN ('novo', 'preparando', 'pronto') )
);

INSERT INTO pedido (cliente, valor) VALUES ('00008756486', 0.0 );
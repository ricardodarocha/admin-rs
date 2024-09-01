CREATE TABLE IF NOT EXISTS planocontas (
    codigo SERIAL NOT NULL,
    id VARCHAR(40) PRIMARY KEY NOT NULL,
    nome TEXT,
    codigopai varchar(40),
    nivel INTEGER,
    conta1 INTEGER,
    conta2 INTEGER,
    conta3 INTEGER,
    conta4 INTEGER,
    conta5 INTEGER,
    empresa INTEGER,
    FOREIGN KEY (codigopai) REFERENCES planocontas(id)
);


CREATE TABLE IF NOT EXISTS compras (
    codigo SERIAL NOT NULL,
    id VARCHAR(40) PRIMARY KEY NOT NULL,
    created_at TIMESTAMP with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    prazo INTEGER,
    fornecedor INTEGER,
    valor REAL,
    data_vencimento DATE,
    valor_pago REAL,
    data_pagamento DATE,
    status TEXT,
    valor_restante REAL,
    empresa INTEGER,
    nota TEXT,
    nome_fornecedor TEXT,
    valor_nota REAL,
    integracao INTEGER,
    refe TEXT DEFAULT '1',
    desconto REAL,
    conta varchar(40) NOT NULL,
    valor_desconto REAL,
    id_empresa VARCHAR(40) not null references empresa(id),
    FOREIGN KEY (conta) REFERENCES planocontas(id)
);

CREATE TABLE IF NOT EXISTS itenscompra (
    codigo SERIAL NOT NULL,
    id VARCHAR(40) PRIMARY KEY NOT NULL,
    compra_id varchar(40),
    produto_id varchar(40),
    preco REAL,
    empresa varchar(40),
    und varchar(40),
    quant REAL,
    FOREIGN KEY (compra_id) REFERENCES compras(id),
    FOREIGN KEY (produto_id) REFERENCES produto(id),
    FOREIGN KEY (empresa) REFERENCES empresa(id)
);


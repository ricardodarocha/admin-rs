-- Add migration script here
-- Criação da tabela charts
CREATE TABLE charts (
    id INTEGER PRIMARY KEY,
    title TEXT,
    labels TEXT
);
-- Criação da tabela series
CREATE TABLE series (
    id INTEGER PRIMARY KEY,
    chart_id INTEGER,
    nome TEXT,
    tipo TEXT,
    valores TEXT,
    backgroundColor TEXT,
    borderColor TEXT,
    borderWidth INTEGER,
    FOREIGN KEY (chart_id) REFERENCES charts (id)
);
-- Inserção de dados de exemplo na tabela charts
INSERT INTO charts (id, title, labels)
VALUES (1, 'Vendas', 'Janeiro,Fevereiro,Março');
INSERT INTO charts (id, title, labels)
VALUES (2, 'Compras', 'Janeiro,Fevereiro,Março');
-- Inserção de dados de exemplo na tabela series
INSERT INTO series (
        chart_id,
        nome,
        tipo,
        valores,
        backgroundColor,
        borderColor,
        borderWidth
    )
VALUES (
        1,
        'Vendas',
        'bar',
        '2500,7944,5533',
        'rgba(255, 99, 132, 0.2);rgba(54, 162, 235, 0.2);rgba(75, 192, 192, 0.2)',
        'rgba(255, 99, 132, 1);rgba(54, 162, 235, 1);rgba(75, 192, 192, 1)',
        1
    );
INSERT INTO series (
        chart_id,
        nome,
        tipo,
        valores,
        backgroundColor,
        borderColor,
        borderWidth
    )
VALUES (    
        2,
        'Compras',
        'bar',
        '3000,6800,5200',
        'rgba(153, 102, 255, 0.2);rgba(255, 159, 64, 0.2);rgba(255, 206, 86, 0.2)',
        'rgba(153, 102, 255, 1);rgba(255, 159, 64, 1);rgba(255, 206, 86, 1)',
        1
    );
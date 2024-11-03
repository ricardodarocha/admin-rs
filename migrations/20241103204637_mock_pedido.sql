-- Insere dados de teste na tabela de pedidos

-- Inserindo alguns clientes
INSERT INTO cliente (id, nome, cidade)
VALUES ('C001', 'Maria Silva', 'São Paulo'),
    ('C002', 'João Oliveira', 'Rio de Janeiro'),
    ('C003', 'Ana Santos', 'Belo Horizonte'),
    ('C004', 'Carlos Souza', 'Porto Alegre'),
    ('C005', 'Fernanda Lima', 'Curitiba');

-- Inserindo pedidos (cobrindo pelo menos cinco meses de dados de vendas)
INSERT INTO pedido (data, cliente, valor, status)
VALUES -- Janeiro
    ('2024-01-15', 'C001', 150.00, 'pronto'),
    ('2024-01-20', 'C002', 200.00, 'pronto'),
    ('2024-01-25', 'C003', 300.00, 'pronto'),
    -- Fevereiro
    ('2024-02-05', 'C001', 120.00, 'pronto'),
    ('2024-02-10', 'C004', 400.00, 'pronto'),
    ('2024-02-15', 'C005', 250.00, 'pronto'),
    -- Março
    ('2024-03-01', 'C003', 180.00, 'pronto'),
    ('2024-03-10', 'C001', 130.00, 'pronto'),
    ('2024-03-20', 'C002', 220.00, 'pronto'),
    -- Abril
    ('2024-04-05', 'C004', 270.00, 'pronto'),
    ('2024-04-15', 'C005', 320.00, 'pronto'),
    ('2024-04-25', 'C003', 190.00, 'pronto'),
    -- Maio
    ('2024-05-05', 'C001', 140.00, 'pronto'),
    ('2024-05-10', 'C004', 350.00, 'pronto'),
    ('2024-05-20', 'C002', 210.00, 'pronto');
-- Pedidos adicionais em meses variados para garantir mais registros
INSERT INTO pedido (data, cliente, valor, status)
VALUES ('2024-03-25', 'C003', 260.00, 'pronto'),
    ('2024-04-30', 'C002', 300.00, 'pronto'),
    ('2024-05-25', 'C005', 410.00, 'pronto');
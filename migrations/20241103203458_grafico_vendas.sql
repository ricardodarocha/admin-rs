drop view if exists vendas_mensais;

CREATE VIEW vendas_mensais AS
SELECT strftime('%Y', data) AS ano,
    strftime('%m', data) AS mes_numero,
    cliente,
    cliente.nome,
    cidade.regiao,
    CASE
        strftime('%m', data)
        WHEN '01' THEN 'Janeiro'
        WHEN '02' THEN 'Fevereiro'
        WHEN '03' THEN 'Março'
        WHEN '04' THEN 'Abril'
        WHEN '05' THEN 'Maio'
        WHEN '06' THEN 'Junho'
        WHEN '07' THEN 'Julho'
        WHEN '08' THEN 'Agosto'
        WHEN '09' THEN 'Setembro'
        WHEN '10' THEN 'Outubro'
        WHEN '11' THEN 'Novembro'
        WHEN '12' THEN 'Dezembro'
    END AS mes,
    SUM(valor) AS total_vendas
FROM pedido
    INNER JOIN cliente on cliente.id = pedido.cliente
    INNER JOIN cidade on cidade.nome = cliente.cidade
WHERE status = 'pronto'
    AND data >= DATE('now', '-12 months')
GROUP BY ano,
    mes_numero,
    cliente,
    cliente.nome,
    regiao
ORDER BY ano DESC,
    mes_numero,
    cliente,
    cliente.nome,
    regiao DESC;
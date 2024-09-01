create table if not exists medida_cliente(
    id_cliente VARCHAR(40) -- Código do cliente
,   nome_cliente VARCHAR -- Nome cliente
,   id_grupo VARCHAR(40) -- Código do grupo
,   nome_grupo VARCHAR -- Nome/descrição do grupo
,   id_rede VARCHAR(40) -- Código da rede
,   nome_rede VARCHAR -- Nome da rede
,   id_categoria VARCHAR(40) -- Código da Categoria
,   nome_categoria VARCHAR -- Nome/descrição da categoria
,   endereco VARCHAR -- Endereço
,   bairro VARCHAR -- Nome do bairro
,   cidade VARCHAR -- Nome da cidade
,   estado VARCHAR -- Nome do estado
,   regiao VARCHAR -- Nome da região
,   pais VARCHAR -- Nome do país
,   cep INTEGER -- CEP
,   data_cadastro  TIMESTAMP with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP-- Data que foi cadastrado
,   status VARCHAR -- Algum tipo de status
,   id_vendedor VARCHAR(40) -- Código do Vendedor
,   id_natureza_financeira VARCHAR(40) -- Código da Natureza Financeira
,   tipo_pessoa VARCHAR -- Pessoa Fisica ou Juridica
,   id_cnpj_cpf VARCHAR(40) -- CNPJ ou CPF
,   tipo_cliente VARCHAR -- Final/Revendedor/Exportador/Produtor Rural, etc
,   ddd INTEGER -- Código do DDD
,   id_telefone VARCHAR(40) -- NÚMERO do telefone
,   risco_credito VARCHAR -- Classificação: A-B-C-D-E-F
,   limite_credito NUMERIC(16, 3) -- Valor do limite de crédito
,   vcto_limite_credito TIMESTAMP -- Data vcto limite crédito
,   nascimento TIMESTAMP -- Data de nascimento/fundação
);

create table if not exists medida_vendedor (
    id_vendedor VARCHAR(40) -- Código do vendedor                               
,    nome_vendedor VARCHAR -- Nome vendedor                   
,    id_supervisor VARCHAR(40) -- Código do supervisor                               
,    nome_supervisor VARCHAR -- Nome do supervisor                       
,    id_gerente VARCHAR(40) -- Código do Gerente                           
,    nome_gerente VARCHAR -- Nome do gerente                   
,    id_equipe VARCHAR(40) -- Código da Equipe                           
,    nome_equipe VARCHAR -- Nome da equipe                   
,    endereco VARCHAR -- Endereço               
,    bairro VARCHAR -- Nome do bairro               
,    cidade VARCHAR -- Nome da cidade               
,    estado VARCHAR -- Nome do estado               
,    pais VARCHAR -- Nome do país              
,    data_cadastro TIMESTAMP with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP-- Data que foi cadastrado                       
,    status VARCHAR -- Algum tipo de status               
);

create table if not exists tipo_meta (
    id_meta varchar(40) not null primary key -- Código meta
,   descricao VARCHAR -- Descrição meta
);

create table if not exists medida_vendas (
    num_pedido serial -- Número do pedido                                  
,   num_nfe serial -- NÚMERO da Nota fiscal                                  
,   id_cliente varchar(40) -- Código do cliente                              
,   id_vendedor varchar(40) -- Código do vendedor                                  
,   id_produto varchar(40) -- Código do produto                              
,   qtd_itens NUMERIC(16, 3) -- Quantidade de produtos vendidos                          
,   preco_unitario NUMERIC(16, 3) -- Valor unitário do produto vendido                              
,   desconto NUMERIC(16, 3) -- Valor do desconto se houver                      
,   comissao NUMERIC(16, 3) -- Comissão do vendedor se houver                      
,   data_pedido TIMESTAMP with time zone not null default current_timestamp -- Data que o cliente foi visitado                                  
,   data_faturamento TIMESTAMP with time zone-- Data que o pedido foi faturado                              
,   peso_bruto NUMERIC(16, 3) -- Peso bruto do pedido                          
,   peso_liquido NUMERIC(16, 3) -- Peso líquido do pedido                          
,   status_pedido VARCHAR -- Algum tipo de status                          
,   data_vencimento TIMESTAMP -- Data de vencimento                                  
,   id_forma_pagto INTEGER -- Código da forma de pagamento                                  
,   data_previsao_entrega TIMESTAMP -- Data que o pedido deve ser entregue                                  
,   data_entrega  TIMESTAMP -- Data que o pedido foi entregue                          
,   ocorrencia_devolucao VARCHAR -- Sim ou Não                                      
);

create table if not exists meta (
    id_vendedor varchar(40) -- Código do vendedor             
,   id_meta varchar(40) -- Código meta             
,   mes INTEGER -- Mês da meta         
,   ano INTEGER -- Ano da Meta         
,    valor NUMERIC(16, 3) -- Valor da meta         
);


create table if not exists medida_fornecedor (
     id_fornecedor varchar(40) -- Código do fornecedor                                  
,    nome_forecedor VARCHAR -- Nome fornecedor                                                         
,    id_grupo varchar(40) -- Código do grupo                      
,    nome_grupo VARCHAR -- Nome/descrição do grupo                     
,    id_rede varchar(40) -- Código da rede                      
,    nome_rede VARCHAR -- Nome da rede                 
,    id_categoria varchar(40) -- Código da Categoria                              
,    nome_categoria VARCHAR -- Nome/descrição da categoria                             
,    endereco VARCHAR -- Endereço                 
,    bairro VARCHAR -- Nome do bairro             
,    cidade VARCHAR -- Nome da cidade             
,    estado VARCHAR -- Nome do estado             
,    regiao VARCHAR -- Nome da região             
,    pais VARCHAR -- Nome do país         
,    cep INTEGER -- CEP              
,    data_cadastro TIMESTAMP with time zone not null default current_timestamp -- Data que foi cadastrado                                
,    status VARCHAR -- Algum tipo de status             
,    id_comprador varchar(40) -- Código do Comprador                              
,    id_natureza_financeira varchar(40) -- Código da Natureza Financeira                                                  
,    tipo_pessoa VARCHAR -- Pessoa Fisica ou Juridica                     
,    id_cnpj_cpf varchar(40) -- CNPJ ou CPF                          
,    id_tipo_fornecedor VARCHAR -- Local/Nacional/Internacional                             
,    ddd INTEGER -- Código do DDD              
,    telefone INTEGER -- NÚMERO do telefone                          
,    risco_credito VARCHAR -- Classificação: A-B-C-D-E-F                         
,    limite_credito NUMERIC(16, 3) -- Valor do limite de crédito                              
,    Vcto_limite_credito TIMESTAMP -- Data vcto limite crédito                                    
);

create table if not exists natureza_financeira (
    id varchar(40)-- Código da natureza financeira                                 
,    natureza_financeira VARCHAR -- Nome da natureza financeira                                
,    nivel VARCHAR -- Analítica ou Sintética                
,    ativo bool -- Sim ou Não                
,    tipo VARCHAR -- Receita ou Despesa                
);

create table if not exists banco (
     id varchar(40) -- Código do banco                                                   
,    nome_banco VARCHAR -- Nome do banco                                               
,    id_agencia varchar(40) -- Código da Agencia                                                       
,    nome_agencia VARCHAR -- Nome da Agencia                                               
,    num_conta VARCHAR -- NÚMERO da Conta                                                   
,    cidade VARCHAR -- Nome da cidade                                           
,    estado VARCHAR -- Nome do estado                                           
,    pais VARCHAR -- Nome do país                                       
,    fluxo_caixa VARCHAR -- Sim ou Não                                                   
,    dias_retencao INTEGER -- Numero de dias de retenção                                                           
,    dias_cobranca INTEGER -- Numero de dias de cobrança                                                           
,    ativo bool -- Sim ou Não                                           
);
-- Add migration script here
create table if not exists forma_pagamento (
    id varchar(40) -- Código da forma de pagamento
,   descricao VARCHAR -- Descrição forma de pagamento
);

create table if not exists pagamento (
     num_titulo VARCHAR(40) -- NÚMERO do título                                  
,    parcela VARCHAR(40) -- NÚMERO da parcela                              
,    id_fornecedor VARCHAR(40) -- Código do fornecedor                                  
,    id_natureza VARCHAR(40) -- Código da natureza financeira                                  
,    id_banco  VARCHAR(40) -- Código do banco                              
,    data_emissao TIMESTAMP with TIME zone not null default CURRENT_TIMESTAMP -- Data de emissão do título                        
,    data_vencimento TIMESTAMP  with TIME zone not null default CURRENT_TIMESTAMP-- Data de vencimento do título                            
,    data_movimento TIMESTAMP  with TIME zone not null default CURRENT_TIMESTAMP-- Data de movimentação do título                            
,    valor_pagamento NUMERIC(16, 3) -- Valor do pagamento                              
,    prazo_pagamento INTEGER -- Prazo de pagamento em dias                                      
,    pagamento_atrasado INTEGER -- Dias de atrazo do pagamento                                          
,    pagamento_antecipado INTEGER -- Dias de antecipação do pagamento                                          
,    id_tipo_titulo VARCHAR(40) -- Tipo do título                         
,    id_fluxo_caixa VARCHAR(40) -- Sim ou Não                             
,    valor_desconto NUMERIC(16, 3) -- Valor do desconto                              
,    valor_multa NUMERIC(16, 3) -- Valor da multa                          
,    valor_juros NUMERIC(16, 3) -- Valor dos juros                          
,    id_status VARCHAR(40) -- Em aberto ou Pago                     
);
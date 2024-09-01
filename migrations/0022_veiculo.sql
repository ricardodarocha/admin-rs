-- Add migration script here
create table if not exists motorista (
    id VARCHAR(40) not null primary key-- Código do motorista               
,    nome_motorista VARCHAR -- Nome motorista                   
,    id_equipe VARCHAR(40) -- Código da Equipe                       
,    nome_equipe VARCHAR -- Nome da equipe               
,    endereco VARCHAR -- Endereço               
,    bairro VARCHAR -- Nome do bairro           
,    cidade VARCHAR -- Nome da cidade           
,    estado VARCHAR -- Nome do estado           
,    pais VARCHAR -- Nome do país           
,    data_cadastro TIMESTAMP with time zone not null default current_timestamp -- Data que foi cadastrado                       
,    status VARCHAR -- Algum tipo de status           
);


create table if not exists rota (
    id VARCHAR(40) not null primary key-- Código da Rota                                                  
,    nome_rota VARCHAR -- Nome da rota                                              
,    data_cadastro TIMESTAMP -- Data que foi cadastrado                                                  
,    id_status VARCHAR(40) -- Algum tipo de status                                          
);

create table if not exists veiculo (
    id VARCHAR(40) not null primary key-- Código do Veículo                                                
,    nome VARCHAR -- Nome do veículo                                   
,    placa VARCHAR -- Placa Veículo                                   
,    id_status_veiculo VARCHAR(40) -- Algum tipo de status do veículo
,    marca VARCHAR -- Marca Veículo                                   
,    ano INTEGER -- Ano Veículo                                            
,    id_tipo_veiculo VARCHAR(40) -- Tipo Veículo                                   
,    bau VARCHAR -- Tipo da carroceria do Veículo                                   
);
create table if not exists tenant  (
    id  VARCHAR(40) not null PRIMARY key,
    cnpj VARCHAR(40) not null,
    cpf VARCHAR(40), 
    status VARCHAR(40) not null, 
    email VARCHAR not null,
    razao_social VARCHAR not null,
    nome VARCHAR ,
    contato VARCHAR(40),
    telefone VARCHAR(40) not null,
    user_id VARCHAR(40) 
    );
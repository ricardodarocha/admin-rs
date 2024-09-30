-- 
create table primeiro_acesso
(
    created timestamp with time zone not null default current_timestamp primary key,
    login varchar not null,
    cnpj varchar,
    segmento varchar,
    email varchar not null,
    telefone varchar not null,
    nome_responsavel varchar not null,
    cpf_responsavel varchar not null,
    senha varchar not null,
    lgpd bool
)

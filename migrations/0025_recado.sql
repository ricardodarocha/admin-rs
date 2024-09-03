-- Add migration script here
create table recado (
    codigo serial primary key,
    id varchar (40),
    id_usuario varchar(40) not null references users(id),
    mensagem text,
    url varchar,
    created_at TIMESTAMP with TIME zone not null default CURRENT_TIMESTAMP,
    readed_at  TIMESTAMP with TIME zone null 

)


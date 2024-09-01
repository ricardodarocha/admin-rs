drop table if exists subscriber;

create table if not exists subscriber (
    id varchar(40) not null primary key,
    nome varchar,
    created TIMESTAMP with TIME ZONE  not null default current_timestamp,
    id_email varchar(40) references contato(id) );
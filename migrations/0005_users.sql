CREATE TABLE IF NOT EXISTS users (
    codigo SERIAL NOT NULL,
	created TIMESTAMP with TIME ZONE not null default current_timestamp,
    id VARCHAR(40) PRIMARY KEY NOT NULL,
    id_empresa VARCHAR(40),
    login VARCHAR  NOT NULL,
    nome VARCHAR NOT NULL,
    password VARCHAR  NOT NULL,
    id_email VARCHAR(40) not null references CONTATO(ID)
);

insert into users (id, login, nome, password, id_email) values (
'0190ef4c-2d28-7371-ba3d-2e7c149ac6b0', '44cd47e1184c', 'Ricardo', '0f81ca8e-2981-5f88-aa95-41c8cbc22a68', '0f81ca8e-2981-5f88-aa95-41c8cbc22a68'), (
'0190ef5d-796e-7a13-8c73-1d0b1e11d5ba', 'caze', 'Cazé', '0bd920dcc8b4c400f0b23af358975cc1', 'f5cde193-1f51-51ae-9b11-d41930704ab3'), (
'0190ef6e-0612-7f23-98b0-ed146428556d', 'demo', 'Vitor (Demonstração)', '', '01911a44-5f65-75c6-9c56-1ce2621f1508'), (
'0190efad-d52a-7e41-8337-e07796f3860b', 'admin', 'Administrador', '0bd920dcc8b4c400f0b23af358975cc1', '6153aa9d-ea52-4e0e-8b50-bc99571a8b26'), (
'6153aa9d-ea52-4e0e-8b50-bc99571a8b26', 'visitante', 'Visitante', '', '01911a44-5f65-75c6-9c56-1ce2621f1508'), (
'INDEFINIDO', 'INDEFINIDO', 'INDEFINIDO', '', 'INDEFINIDO');


create table log_acesso (
	created TIMESTAMP with TIME ZONE not null primary key default current_timestamp,
    id_usuario varchar(40) not null references users(id),
	login varchar not null,
	status varchar(40) not null,
    origem varchar not null
);


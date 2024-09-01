-- Add migration script here
create table servico_api (
	id varchar(40) not null primary key,
	nome varchar(40) not null
);

create table requisicao (
	id varchar(40) not null primary key,
	id_servico_api varchar(40) not null references servico_api(id),
	origim varchar(40),
	header text,
	body text,
	escope text,
	response text,
	status varchar(40),
	id_usuario varchar(40) not null references users(id),
	id_empresa varchar(40) not null references empresa(id), 
	created TIMESTAMP with TIME zone not null default current_timestamp
);

CREATE TABLE auditoria (
	id varchar(40) NOT NULL,
	created timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	id_empresa varchar(40) NOT NULL,
	id_usuario varchar(40) NOT NULL,
	id_perfil_usuario varchar(40) NOT NULL,
	tabela varchar(40) NULL,
	valor_antigo text NULL,
	valor_novo text NULL,
	operacao varchar(40) NULL,
	CONSTRAINT auditoria_pkey PRIMARY KEY (id),
	CONSTRAINT auditoria_id_empresa_fkey FOREIGN KEY (id_empresa) REFERENCES empresa(id),
	CONSTRAINT auditoria_id_perfil_usuario_fkey FOREIGN KEY (id_perfil_usuario) REFERENCES perfil_usuario(id),
	CONSTRAINT auditoria_id_usuario_fkey FOREIGN KEY (id_usuario) REFERENCES users(id)
);

-- Drop table

-- DROP TABLE public.log_acesso;

CREATE TABLE log_acesso (
	created timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	login varchar NULL,
	status varchar(40) NULL,
	origem varchar NULL,
	id_usuario varchar(40) NOT NULL DEFAULT 'INDEFINIDO'::character varying,
	CONSTRAINT log_acesso_pkey PRIMARY KEY (created),
	CONSTRAINT log_acesso_id_usuario_fkey FOREIGN KEY (id_usuario) REFERENCES users(id)
);

create table rua(
    	id varchar(40) not null primary key,
	    nome varchar not null
	);    
	
	create table bairro(
    	id varchar(40) not null primary key,
	    nome varchar not null
	);    
	
	drop table if exists estado ;
	create table estado(
    codigo SERIAL  primary key,
    	id varchar(40) not null,
	    nome varchar not null,
    siglauf CHAR(2) NOT NULL
	);


CREATE TABLE cidade (
    codigo SERIAL PRIMARY KEY,
    codigoestado INTEGER REFERENCES estado(codigo) ON DELETE CASCADE,
    nome VARCHAR(255) NOT NULL,
    codigoibge INTEGER UNIQUE NOT NULL
);

    
create table endereco (id varchar(40) not null primary key,
    id_logradouro varchar(40) not null default 'RUA',
    id_rua varchar(40) not null references rua(id),
    id_bairro varchar(40) not null references bairro(id),
    numero varchar(40),
    cep varchar(40),
    codigocidade integer references cidade(codigo) on delete cascade,
    codigoestado INTEGER REFERENCES estado(codigo) ON DELETE CASCADE,
    complemento varchar);
	
DROP TABLE if exists status_pedido cascade;

CREATE TABLE if not exists status_pedido (
	id varchar(40) NOT NULL,
	descricao varchar NOT NULL,
	CONSTRAINT status_pedido_pkey PRIMARY KEY (id)
);


    insert into status_pedido VALUES
    ('4a657b6e-ed8c-509d-ae00-3923bf5482c3', 'NOVO' ),
    ('e2b319a3-a428-5ae3-aefd-a9dea8f5e692', 'ACEITO' ),
    ('4032ee09-7769-5be8-853e-11e1c3ab56ae', 'RECUSADO' ),
    ('f7c28e9f-838b-5336-bd16-eddbe37358b3', 'FINALIZADO' );

    
drop table if exists tipo_evento_pedido cascade;	
create table if not exists tipo_evento_pedido (
	id varchar(40) NOT NULL,
	descricao varchar NOT NULL,
	CONSTRAINT tipo_evento_pedido_pkey PRIMARY KEY (id)
);
    insert into tipo_evento_pedido VALUES
    ('784772bf-e2f6-53dc-a39e-6548f3dca9ce', 'CRIADO' ),
    ('dc899daa-8c9f-5fa9-bd06-257f79cb8bac', 'REVISADO' ),
    ('00d41917-19e7-5a7b-9d9c-5d639499cbf2', 'CANCELADO' ),
    ('a21f3d5f-33ab-5ad8-b2b7-31953a666cf5', 'EXCLUIDO' ),
    ('d8973d45-4d41-5797-8788-4d30101495f5', 'FINALIZADO' );

    

drop table if exists evento_pedido;

create table if not exists evento_pedido  (
	id varchar(40) NOT NULL,
	created TIMESTAMP with TIME ZONE not null default current_timestamp,
	id_responsavel varchar(40) not null references pessoa(id),
	descricao varchar,
	id_tipo_evento_pedido varchar(40) not null references tipo_evento_pedido(id),
	id_evento_pedido_anterior varchar(40) null references evento_pedido(id),
	id_evento_pedido_proximo varchar(40) null references evento_pedido(id),
	CONSTRAINT evento_pedido_pkey PRIMARY KEY (id)
);

create table if not exists pedido(
	codigo serial, 
	numero int4 not null default 0,
	id varchar(40) not null primary key,
	id_empresa varchar(40) NULL references empresa(id),
	id_usuario varchar(40) NOT NULL references users(id),
    id_cliente VARCHAR(40) NULL references pessoa(id), 
	created TIMESTAMP with TIME ZONE not null default current_timestamp,
	valor numeric(16,3),
	data_entrega TIMESTAMP with TIME ZONE not null default current_timestamp,
	id_responsavel varchar(40) null references pessoa(id),
	id_status_pedido varchar(40) not null references status_pedido(id) default '4a657b6e-ed8c-509d-ae00-3923bf5482c3' --novo
);

create table if not exists  item_pedido ( 
  id_empresa VARCHAR(40) not null references empresa(id), --allow redundant
  id_cliente VARCHAR(40) not null references pessoa(id), --allow redundant
  id_pedido VARCHAR(40) not null references pedido(id),
  id_item serial not null,
  id_produto VARCHAR(40) not null references produto(id),
  created TIMESTAMP with TIME ZONE not null default current_timestamp,
  preco numeric(16,2),
  quantidade numeric(16,2),
  constraint pk_item_pedido primary key (id_pedido, id_item)
);
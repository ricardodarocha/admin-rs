
create table if not exists grupo_produto  (id varchar(40) not null primary key, 
nome varchar );

insert into grupo_produto values 
( 'ALIMENTOS', 'ALIMENTOS'),
( 'BEBIDAS', 'BEBIDAS'),
( 'EMBALAGENS', 'EMBALAGENS'),
( 'PAPELARIA', 'PAPELARIA'),
( 'MOVEIS', 'MOVEIS'),
( 'ESTOFADOS', 'ESTOFADOS'),
( 'COMBUSTIVEIS', 'COMBUSTIVEIS'),
( 'INSUMOS', 'INSUMOS'),
( 'ELETRICOS', 'MATERIAIS ELÉTRICOS'),
( 'HIDRAULICOS', 'MATERIAIS HIDRÁULICOS'),
( 'FERRAMENTAS', 'FERRAMENTAS'),
( 'VESTUARIO', 'VESTUÁRIO'),
( 'CALCADOS', 'CALÇADOS')
;

create table if not exists  categoria_produto (id varchar(40) not null primary key, 
nome varchar );

create table if not exists  unidade (id varchar(40) not null primary key, 
simbolo varchar(40),
nome varchar );


CREATE TABLE if not exists produto (
	codigo serial NOT NULL,
	id varchar(40) NOT NULL,
	nome text NOT NULL,
	icone varchar not null default '🔵  ',
	jclasses text NULL,
	jespecificacoes text NULL,
	jvariacoes text NULL,
	tipo text NOT NULL DEFAULT 'produto'::text,
	id_empresa varchar(40) not null references empresa(id),
	descricao text NULL,
	id_grupo_produto varchar(40) not null references grupo_produto(id),
	id_categoria_produto varchar(40) not null references categoria_produto(id),
	classe int4 NULL,
	segmento int4 NULL,
	setor int4 NULL,
	codbarras text NULL,
	url text NULL,
	memorando text NULL,
	preco float4 NOT NULL,
	custo float4 NULL,
	estoque float4 NULL,
	maximo float4 NULL,
	minimo float4 NULL,
	ideal float4 NULL,
	ativo bool NOT NULL DEFAULT true,
	referencia text NULL,
	abc text NOT NULL DEFAULT ' '::text,
	composicao bool NOT NULL DEFAULT false,
	medida float4 NULL,
	und text NOT NULL DEFAULT 'un'::text,
	componente bool NOT NULL DEFAULT true,
	compra bool NOT NULL DEFAULT true,
	venda bool NOT NULL DEFAULT true,
	preco_compra float4 NULL,
	id_unidade varchar(40) NULL references unidade(id) ,
	formato varchar NULL , --exemplo Pacote 5 Kg => {formato: "Pacote", tamanho: "5", id_unidade: "kg"}
	tamanho varchar NULL , --exemplo 5 Kg => {tamanho: "5", id_unidade: "kg"}
	und_compra text NULL,
	und_venda text NULL,
	fator_compra float4 NULL,
	fator_venda float4 NULL,
	nometemp text NULL,
	redireciona varchar(40) NULL,
	created timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT produto_pkey PRIMARY KEY (id),
	CONSTRAINT produto_unique_name UNIQUE (nome, id_empresa),
	CONSTRAINT produto_redireciona_fkey FOREIGN KEY (redireciona) REFERENCES produto(id)
);

create table grupo_produto_empresa (
  id_empresa varchar(40) not null references empresa(id) ,
  id_grupo_produto varchar(40) not null references grupo_produto(id),
  ativo bool not null default true);
  
 insert into grupo_produto_empresa(id_empresa, id_grupo_produto)
 select empresa.id, grupo_produto.id from empresa, grupo_produto ;

drop table if exists unidade cascade;

create table if not exists unidade (id varchar(40) not null PRIMARY KEY, descricao varchar);

insert into unidade (id, descricao) VALUES
('Kg', 'Kilograma'), 
('g', 'Grama'), 
('Un', 'Unidade');

create table  if not exists produto_unidade (id_produto varchar(40) not null references produto(id),
id_unidade varchar(40) references unidade(id),
constraint pk_produto_unidade primary key (id_produto, id_unidade));

 alter table empresa add avatar varchar;
 alter table produto add avatar varchar;
 alter table pessoa add avatar varchar;
 alter table users add avatar varchar;

 update empresa set avatar = 'https://static.vecteezy.com/system/resources/thumbnails/048/116/111/small/camera-icon-photo-camera-sign-and-symbol-free-png.png';
update produto set avatar = 'https://static.vecteezy.com/system/resources/thumbnails/048/116/111/small/camera-icon-photo-camera-sign-and-symbol-free-png.png';
update users set avatar = 'https://media.istockphoto.com/id/1495088043/vector/user-profile-icon-avatar-or-person-icon-profile-picture-portrait-symbol-default-portrait.jpg?s=612x612&w=0&k=20&c=dhV2p1JwmloBTOaGAtaA3AW1KSnjsdMt7-U_3EZElZ0=';
update pessoa set avatar = 'https://static.vecteezy.com/system/resources/thumbnails/048/116/111/small/camera-icon-photo-camera-sign-and-symbol-free-png.png';
 




       alter table item_pedido add campo1 numeric(16,6);
       alter table item_pedido add campo2 numeric(16,6);
       alter table item_pedido add campo3 numeric(16,6);
       alter table item_pedido add campo4 numeric(16,6);
       alter table item_pedido add campo5 numeric(16,6);
       alter table item_pedido add campo6 numeric(16,6);
       alter table item_pedido add campo7 numeric(16,6);
       alter table item_pedido add campo8 numeric(16,6);
       alter table item_pedido add campo9 numeric(16,6);
       alter table itempedido add nome_campo1 varchar;
       alter table itempedido add nome_campo2 varchar;
       alter table itempedido add nome_campo3 varchar ;
       alter table itempedido add nome_campo4 varchar ;
       alter table itempedido add nome_campo5 varchar ;
       alter table itempedido add nome_campo6 varchar ;
       alter table itempedido add nome_campo7 varchar ;
       alter table itempedido add nome_campo8 varchar ;
       alter table itempedido add nome_campo9 varchar ;
       alter table itempedido add id_unidade1 varchar(40) references unidade(id);
       alter table itempedido add id_unidade2 varchar(40) references unidade(id) ;
       alter table itempedido add id_unidade3 varchar(40) references unidade(id);
       alter table itempedido add id_unidade4 varchar(40) references unidade(id) ;
       alter table itempedido add id_unidade5 varchar(40) references unidade(id) ;
       alter table itempedido add id_unidade6 varchar(40) references unidade(id) ;
       alter table itempedido add id_unidade7 varchar(40) references unidade(id);
       alter table itempedido add id_unidade8 varchar(40) references unidade(id);
       alter table itempedido add id_unidade9 varchar(40) references unidade(id);
       
      
       alter table produto add campo1 numeric(16,6);
       alter table produto add campo2 numeric(16,6);
       alter table produto add campo3 numeric(16,6); 
       alter table produto add campo4 numeric(16,6);
       alter table produto add campo5 numeric(16,6);
       alter table produto add campo6 numeric(16,6);
       alter table produto add campo8 numeric(16,6);
       alter table produto add campo7 numeric(16,6);
       alter table produto add campo9 numeric(16,6);
       alter table produto add nome_campo1 varchar;
       alter table produto add nome_campo2 varchar;
       alter table produto add nome_campo3 varchar ;
       alter table produto add nome_campo4 varchar ;
       alter table produto add nome_campo5 varchar ;
       alter table produto add nome_campo6 varchar ;
       alter table produto add nome_campo7 varchar ;
       alter table produto add nome_campo8 varchar ;
       alter table produto add nome_campo9 varchar ;
       alter table produto add id_unidade1 varchar(40) references unidade(id);
       alter table produto add id_unidade2 varchar(40) references unidade(id) ;
       alter table produto add id_unidade3 varchar(40) references unidade(id);
       alter table produto add id_unidade4 varchar(40) references unidade(id) ;
       alter table produto add id_unidade5 varchar(40) references unidade(id) ;
       alter table produto add id_unidade6 varchar(40) references unidade(id) ;
       alter table produto add id_unidade7 varchar(40) references unidade(id);
       alter table produto add id_unidade8 varchar(40) references unidade(id);
       alter table produto add id_unidade9 varchar(40) references unidade(id);


       
       alter table produto add decimais1 integer not null default 3;
       alter table produto add decimais2 integer not null default 3;
       alter table produto add decimais3 integer not null default 3;
       alter table produto add decimais4 integer not null default 3;
       alter table produto add decimais5 integer not null default 3;
       alter table produto add decimais6 integer not null default 3;
       alter table produto add decimais8 integer not null default 3;
       alter table produto add decimais7 integer not null default 3;
       alter table produto add decimais9 integer not null default 3;

       
       alter table item_pedido  add decimais1 integer not null default 3;
       alter table item_pedido  add decimais2 integer not null default 3;
       alter table item_pedido  add decimais3 integer not null default 3;
       alter table item_pedido  add decimais4 integer not null default 3;
       alter table item_pedido  add decimais5 integer not null default 3;
       alter table item_pedido  add decimais6 integer not null default 3;
       alter table item_pedido  add decimais8 integer not null default 3;
       alter table item_pedido  add decimais7 integer not null default 3;
       alter table item_pedido  add decimais9 integer not null default 3;

      create table fabricante_produto (id_fabricante varchar(40) not null references pessoa(id) ,
      id_produto varchar(40) not null references produto(id), created_at TIMESTAMP not null default current_timestamp ,
      ativo bool not null default true);
     
      create table marca_produto (id_marca varchar(40) not null,
      id_produto varchar(40) not null references produto(id), created_at TIMESTAMP not null default current_timestamp ,
      ativo bool not null default true);
      
      alter table grupo_produto add classe varchar not null default 'bg-warning';
      alter table categoria_produto add classe varchar not null default 'bg-warning';
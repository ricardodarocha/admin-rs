create table tipo_variacao (id varchar(40) not null primary key);

insert into tipo_variacao values ('COR'),('TECIDO'),
('TAMANHO'),('TIPO'),('ACABAMENTO'),
('OPCAO 1'),('OPCAO 2'),('OPCAO 3');


create table tipo_GRADE (id varchar(40) not null primary key);

insert into tipo_GRADE values ('ADULTO'),('INFANTIL'), ('INDEFINIDO');

drop table if exists variacao_produto; 

create table if not exists  VARIACAO_PRODUTO
(id_variacao varchar(40) not null references tipo_variacao,
 id_produto varchar(40) null references produto(id),
 id_empresa varchar(40) null references empresa(id),
 id_grade varchar(40) null references tipo_grade(id),
 valor varchar, 
 constraint PK_VARIACAO_PRODUTO primary key (ID_VARIACAO, ID_PRODUTO, ID_EMPRESA, ID_GRADE, VALOR)
 
);

insert into CATEGORIA_produto (ID) values ('INDEFINIDO');
insert into GRUPO_produto (ID) values ('INDEFINIDO');

insert into EMPRESA (ID, NOME, fantasia, ENDERECO,
CIDADE, ESTADO) values ('INDEFINIDO', 'INDEFINIDA', 'INDEFINIDA', 'INDEFINIDO', 'INDEFINIDA', 'IN');

insert into produto (id, nome, PRECO,  id_grupo_produto , id_categoria_produto, ID_EMPRESA ) values ('0', 'INDEFINIDO', 0, 'INDEFINIDO', 'INDEFINIDO', 'INDEFINIDO');

insert into variacao_produto  (ID_VARIACAO, VALOR, ID_PRODUTO, ID_EMPRESA, ID_GRADE) values 
('COR', 'BRANCO', '0', 'INDEFINIDO', 'INDEFINIDO')
,('COR', 'AZUL', '0', 'INDEFINIDO', 'INDEFINIDO')
,('COR', 'MARROM', '0', 'INDEFINIDO', 'INDEFINIDO')
,('COR', 'PRETO', '0', 'INDEFINIDO', 'INDEFINIDO')
,('COR', 'BEGE', '0', 'INDEFINIDO', 'INDEFINIDO')
,('ACABAMENTO', 'FOSCO', '0', 'INDEFINIDO', 'INDEFINIDO')
,('ACABAMENTO',  'BORDADO', '0', 'INDEFINIDO', 'INDEFINIDO')
,('ACABAMENTO', 'BRILHO', '0', 'INDEFINIDO', 'INDEFINIDO')
,('TECIDO',  'LINHO', '0', 'INDEFINIDO', 'INDEFINIDO')
,('TECIDO',  'SUED', '0', 'INDEFINIDO', 'INDEFINIDO')
,('TAMANHO', '40', '0', 'INDEFINIDO', 'INDEFINIDO')
,('TAMANHO', '38', '0', 'INDEFINIDO', 'INDEFINIDO')
,('TAMANHO',  'GG', '0', 'INDEFINIDO', 'INDEFINIDO')
,('TAMANHO',  'XG', '0', 'INDEFINIDO', 'INDEFINIDO')
,('TAMANHO', 'M', '0', 'INDEFINIDO', 'INDEFINIDO')
,('TAMANHO', 'P', '0', 'INDEFINIDO', 'INDEFINIDO')
,('TAMANHO', 'PP', '0', 'INDEFINIDO', 'INDEFINIDO');
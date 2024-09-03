
drop table if exists tipo_pessoa; 
create table if not exists tipo_pessoa  (
    id  VARCHAR(40) not null PRIMARY key,
    simbolo VARCHAR NOT NULL, --PF PJ OU , 
    nome VARCHAR NOT NULL--PESSOA FISICA, PESSOA JURIDICA, OUTRA, 
    );
   
insert into tipo_pessoa values 
  ('879e07b1-9344-50c6-811a-3b1d52c76650', 'PF', 'PESSOA FISICA'),
  ('cc11f8f3-b7a3-55c5-97a5-6cc3ead1bbc0', 'PJ', 'PESSOA JURIDICA'),
  ('d88e48cd-1560-5952-9ab0-03e96c6e5750', 'OU', 'OUTRA') ,
  ('INDEFINIDO', 'IN', 'INDEFINIDO') ;

drop table if exists tipo_identificacao cascade; 
create table if not exists tipo_identificacao  (
    id  VARCHAR(40) not null PRIMARY key,
    simbolo VARCHAR NOT NULL ,--CPF CNPJ CNH CNS PIS PASEP CT OAB CRC CRM CRE CREA , 
    nome VARCHAR NOT NULL--Cadastro de Pessoa Fisica, Cadastro Nacional de Pessoa Juridica, Cartão Nacional de Saude Pis, 
    );
   
insert into tipo_identificacao values 
  ('1be5b89f-a358-5012-9fd1-c5b209bce601', 'CPF', 'CADASTRO DE PESSOA FISICA'),
  ('3ce43606-35a0-5210-ac94-fd1df93caf3c', 'CNPJ', 'CADASTRO NACIONAL DE PESSOA JURIDICA'),
  ('81256fee-f72c-5a5f-a7bd-28379ce8f72e', 'CNH', 'CARTEIR NACIONAL DE HABILITACAO'),
  ('B2b6e76c-b64b-5c3f-933e-82b7a0c5bab4', 'CNS', 'CARTÃO NACIONAL DE SAÚDE'),
  ('efec5b38-2ae5-534b-88ae-73b86e730ce7', 'PIS', 'PIS'),
  ('0c5baf00-ce69-5c30-9c5c-748c3da3caa6', 'PASEP', 'PASEP'),
  ('15ff47e3-3dcc-583a-94b4-410d386d58b4', 'CT', 'CARTEIRA DE TRABALHO'),
  ('ae26b238-7e77-5b3c-9305-45734d03bf06', 'OAB', 'ORDEM DOS ADVOGADOS DO BRASIL'),
  ('468fc9d4-063e-5a41-b1d6-ee34d1cfae5b', 'CRC', 'CONSELHO REGIONAL DE CONTABILIDADE'),
  ('7e095002-33fa-5114-80a1-a89285a35323', 'CRM', 'CONSELHO REGIONAL DE MEDICINA'),
  ('af425acd-8d07-5734-bcf8-a12b7765dddc', 'CREA', 'CONSELHO REGIONAL DE ENGENHARIA')
  ;

 create table if not exists identificacao (
    id  VARCHAR(40) not null PRIMARY key,
    id_tipo_identificacao  varchar(40) not null references tipo_identificacao(id),
    descricao  varchar not null
    );

insert into identificacao values ('47dad86f-1d39-57fa-ab12-9ecf60007516', '3ce43606-35a0-5210-ac94-fd1df93caf3c', '0001-00');

drop table if exists status_pessoa cascade;

 create table if not exists status_pessoa (
    id  VARCHAR(40) not null PRIMARY key,
    descricao  varchar not null
    );

    insert into status_pessoa VALUES
    ('54c18fe3-ce77-5184-8394-330e1ae3d7ae', 'LEAD' ),
    ('1e69cb55-331f-5b74-a7bd-f96d4f25e81d', 'CREDENCIADO' ),
    ('8143636b-0cc4-5657-a8cd-f14571be9187', 'REVOGADO' );

drop table if exists grupo_pessoa cascade;

 create table if not exists grupo_pessoa (
    id  VARCHAR(40) not null PRIMARY key,
    nome  varchar not null
    );

    insert into grupo_pessoa VALUES
    ('INDEFINIDO', 'INDEFINIDO' );
    
    drop table if exists regiao_pessoa cascade;

 create table if not exists regiao_pessoa (
    id  VARCHAR(40) not null PRIMARY key,
    descricao  varchar not null
    );

    insert into regiao_pessoa VALUES
    ('INDEFINIDO', 'INDEFINIDO' );

drop table if exists perfil_pessoa cascade;

 create table if not exists perfil_pessoa (
    id  VARCHAR(40) not null PRIMARY key,
    descricao  varchar not null
    );

    insert into perfil_pessoa VALUES
    ('INDEFINIDO', 'INDEFINIDO' );

drop table if exists nacionalidade_pessoa cascade;

 create table if not exists nacionalidade_pessoa (
    id  VARCHAR(40) not null PRIMARY key,
    descricao  varchar not null
    );

    insert into nacionalidade_pessoa VALUES
    ('BRASILEIRO', 'BRASILEIRO' ),
    ('EXTRANGEIRO', 'EXTRANGEIRO' ),
    ('INDEFINIDO', 'INDEFINIDO' );

create table if not exists pessoa  (
    id  VARCHAR(40) not null PRIMARY key,
    codigo serial,
    created TIMESTAMP with TIME ZONE not null default current_timestamp,
    alterado TIMESTAMP with TIME ZONE not null default current_timestamp,
    id_empresa VARCHAR(40) not null references empresa(id),
    id_tipo_pessoa VARCHAR(40) not null references tipo_pessoa(id),
    id_grupo_pessoa VARCHAR(40) not null references grupo_pessoa(id),
    id_regiao_pessoa VARCHAR(40) not null references regiao_pessoa(id),
    id_perfil_pessoa VARCHAR(40) not null references perfil_pessoa(id),
    id_nacionalidade_pessoa VARCHAR(40) not null references nacionalidade_pessoa(id),
    observacoes text,
    id_identificacao VARCHAR(40) not null references identificacao(id), 
    id_tipo_identificacao VARCHAR(40) not null references tipo_identificacao(id), 
    id_status VARCHAR(40) not null references status_pessoa(id) DEFAULT '54c18fe3-ce77-5184-8394-330e1ae3d7ae', 
    id_email VARCHAR not null references contato(id),
    razao_social VARCHAR not null,
    nome VARCHAR not null,
    id_telefone VARCHAR(40) not null references contato(id),
    id_celular VARCHAR(40) null references contato(id),
    url varchar,
    avatar varchar,
    abc varchar(1) not null default '_',
    ativo bool not null default true,
    --create table endereco_pessoa tipo_endereco (comercial, residencial), id_endereco varchar(40) references endereco(id)
    id_endereco_principal varchar(40) not null references endereco(id),
    id_endereco_financeiro varchar(40) null references endereco(id),
    id_endereco_comercial varchar(40) null references endereco(id),
    id_endereco_cobranca varchar(40) null references endereco(id),
    cliente bool not null default true,
    fornecedor bool not null default true,
    funcionario bool not null default true,
    motorista bool not null default true,
    transportador bool not null default true,
    contador bool not null default true,
    paciente bool not null default true,
    vendedor bool not null default true,  
    id_user VARCHAR(40) 
    );

alter table grupo_pessoa add classe varchar not null default 'bg-warning';
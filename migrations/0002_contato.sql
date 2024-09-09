drop table if exists tipo_contato  cascade; 

create table if not exists tipo_contato  (
    id  VARCHAR(40) not null PRIMARY key,
    nome VARCHAR not null --telefone, email, 
    );
   
insert into tipo_contato (id, nome) values 
  ('a0a66748-1a9e-5b95-a418-7270bf5caba8', 'TELEFONE'),
  ('4920966b-d6fb-51a2-8218-7ede4eb28d0c', 'EMAIL'),
  ('bb7a936b-faf3-5814-ac8c-1412172019fb', 'WHATSAPP'),
  ('5f348c82-c947-5c25-98a6-7324c5081635', 'INSTAGRAM'),
  ('36273362-32b2-59f7-b0f4-e3805e3c6148', 'SKYPE'),
  ('0e09a04c-b4e6-5a1d-a12b-f34559323a8e', 'TELEGRAM'),
  ('37245b38-f5d6-5f1d-bad9-5cd481fa93be', 'DISCORD'),
  ('INDEFINIDO', 'INDEFINIDO') ;

create table if not exists contato  (
    id  VARCHAR(40) not null PRIMARY key,
    id_tipo_contato VARCHAR(40) not null references tipo_contato(id),
    descricao VARCHAR 
    );


--fixa alguns contatos 
insert into contato values (
'f5cde193-1f51-51ae-9b11-d41930704ab0',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'admin@acme.com'), (
'f5cde193-1f51-51ae-9b11-d41930704ab3',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'sac.venturi@gmail.com'), (
'0f81ca8e-2981-5f88-aa95-41c8cbc22a68',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'ricardodarocha@outlook.com'), (
'6153aa9d-ea52-4e0e-8b50-bc99571a8b26',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'admin@acme.com'), (
'01911a44-5f65-75c6-9c56-1ce2621f1508',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'demo@sistema.com'), (
'01911a44-5f65-7f4f-9de1-cefc9b4b78bb',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'teste@sistema.com') (
'INDEFINIDO',	'INDEFINIDO',	'INDEFINIDO')
    ;
-- estes campos que começam com _ recebem o nome da rua sem acentos e tudo em maiúsculo para facilitar a busca exata
alter table rua add _nome varchar not null default '';
alter table bairro add _nome varchar not null default '';
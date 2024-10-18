alter table rua add constraint if not exists U_rua__nome UNIQUE(_nome);
alter table bairro add constraint if not exists U_bairro__nome UNIQUE(_nome);
alter table endereco drop constraint if exists U_endereco;
alter table endereco add constraint U_endereco UNIQUE(id_rua, id_bairro, id_cidade, id_estado, cep);

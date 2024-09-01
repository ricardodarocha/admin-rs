
insert into empresa  (id, nome, fantasia, endereco, cidade, estado, id_cnpj) 
  values (
  'e409000e-63d2-53e5-859a-f0f88bac8730', 
  'DEMONSTRACAO', 
  'Empresa Demonstracao', 
  'Av Beira Rio', 
  'Uba', 
  'MG',
  (select id from identificacao where descricao = '0001-00' ));

create table empresa_usuario
(
id_empresa VARCHAR(40) not null references empresa(id),
id_usuario VARCHAR(40) not null references users(id),
created TIMESTAMP with TIME ZONE not null default current_timestamp,
CONSTRAINT empresa_usuario_pkey PRIMARY KEY (id_empresa, id_usuario)
);
   
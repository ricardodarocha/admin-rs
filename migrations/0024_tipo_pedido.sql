create table tipo_pedido (id varchar(40) not null primary key , descricao varchar not null);
   insert into tipo_pedido values ('MODELO', 'Pedido modelo');
   insert into tipo_pedido values ('COMPRA', 'Pedido de Compra');
   insert into tipo_pedido values ('PEDIDO', 'Pedido de Venda');
   insert into tipo_pedido values ('TESTE', 'Pedido de Teste');
   insert into tipo_pedido values ('DEMONSTRACAO', 'Pedido de Demonstração');

alter table pedido add id_tipo_pedido varchar(40) not null references tipo_pedido(id) default 'PEDIDO';
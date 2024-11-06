-- Add migration script here
alter table produto add nome varchar;

update produto set nome = descricao;

insert into produto (id, preco, nome, descricao) values 

('P001', 22.00, 'X Burguer',   'Pão, hambúrguer bovino, queijo, alface, tomate e maionese especial'),  
('P002', 23.00, 'Hamburguer',   'Pão, hambúrguer bovino, queijo, alface, tomate e maionese especial'),  
('P003', 26.00, 'Egg - Burguer',   'Pão, hambúrguer bovino, queijo, alface, tomate e maionese especial'),  
('P004', 31.00, 'X Tudo',   'Pão, hambúrguer bovino, queijo, alface, tomate e maionese especial'),  
('P005', 32.00, 'X Egg Salada',   'Pão, hambúrguer bovino, queijo, alface, tomate e maionese especial'),  
('P006', 30.00, 'Frango Burguer',   'Pão, hambúrguer bovino, queijo, alface, tomate e maionese especial'),  
('P007', 18.00, 'Pork Burguer',   'Pão, hambúrguer bovino, queijo, alface, tomate e maionese especial'),  
('P008', 12.00, 'Batata Frita',   'Pão, hambúrguer bovino, queijo, alface, tomate e maionese especial'),  
('P009', 6.00, 'Refrigerante',   'Pão, hambúrguer bovino, queijo, alface, tomate e maionese especial');

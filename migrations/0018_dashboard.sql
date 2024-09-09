-- Add migration script here
create table if not exists dashboard (
    id_usuario varchar(40) not null references users(id),
    id_grupo_dashboard varchar(40) not null default 'main',
    valor varchar(40) not null default '0',
    titulo varchar(40) not null default 'Demonstração',
    descricao varchar(40),
    atualizado TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    avatar VARCHAR
);

--Insere pelo menos um card de dashboard para o usuário
insert into dashboard (id_usuario, id_grupo_dashboard, titulo) 
select users.id, 'main', 'Pedidos' from users;
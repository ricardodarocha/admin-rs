
--os perfis do sistema
create table if not exists perfil_usuario (
    id  VARCHAR(40) not null PRIMARY key,
	created TIMESTAMP with TIME ZONE not null default current_timestamp,
    nome VARCHAR
);

insert into perfil_usuario (id, nome) VALUES
('402415cc-5b1b-50b0-b2dc-bbcdde013815', 'DEV'), --developer
('770aed69-191c-5b14-b816-572d02d61564', 'AMIN'), --administrador
('6d70c310-d672-56cc-8aaa-32ef2bbb8f20', 'SUPER'), --supervisor
('edbde215-0f9d-5e02-a252-1a6290568fea', 'USER'), --usuario comum
('a6cb735a-e834-5000-9bf5-2d1ea59161d6', 'VISITOR') --visitante
;

create table if not exists perfil_usuario_empresa (
    id_empresa VARCHAR(40) not null references empresa(id),
    id_perfil_usuario VARCHAR(40) not null references perfil_usuario(id),
	created TIMESTAMP with TIME ZONE not null default current_timestamp,
    nome VARCHAR null, --override
	CONSTRAINT perfil_usuario_empresa_pkey PRIMARY KEY (id_empresa, id_perfil_usuario)
); 

--Configura todas as empresas com todos os perfils
--Ao adicionar uma empresa, deverá ser refeito para a empresa criada
insert into perfil_usuario_empresa (id_empresa , id_perfil_usuario , nome)
select empresa.id, perfil_usuario.id, perfil_usuario.nome from empresa, perfil_usuario 
where not exists (select id_empresa from perfil_usuario_empresa);



insert into perfil_usuario_empresa (id_empresa, id_perfil_usuario)
select empresa.id, perfil_usuario.id from empresa, perfil_usuario;

--o perfil que foi atribuido a cada usuario
create table usuario_perfil
(
id_usuario VARCHAR(40) not null references users(id),
id_perfil_usuario VARCHAR(40) not null references perfil_usuario(id),
id_usuario_admin VARCHAR(40) not null references users(id),
created TIMESTAMP with TIME ZONE not null default current_timestamp,
CONSTRAINT usuario_perfil_pkey PRIMARY KEY (id_usuario, id_perfil_usuario)
);

create table auditoria (
   id  VARCHAR(40) not null PRIMARY key,
   created TIMESTAMP with TIME ZONE not null default current_timestamp,
   id_empresa VARCHAR(40) not null references empresa(id),
   id_usuario VARCHAR(40) not null references users(id),
   id_perfil_usuario VARCHAR(40) not null references perfil_usuario(id),
   tabela varchar(40),
   valor_antigo text,
   valor_novo text,
   operacao varchar(40)   
   );
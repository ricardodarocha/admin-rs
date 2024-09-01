CREATE TABLE if not exists usuario_perfil (
	id_usuario varchar(40) NOT NULL,
	id_perfil_usuario varchar(40) NOT NULL,
	id_usuario_admin varchar(40) NOT NULL,
	created timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT usuario_perfil_pkey PRIMARY KEY (id_usuario, id_perfil_usuario),
	CONSTRAINT usuario_perfil_id_perfil_usuario_fkey FOREIGN KEY (id_perfil_usuario) REFERENCES perfil_usuario(id),
	CONSTRAINT usuario_perfil_id_usuario_admin_fkey FOREIGN KEY (id_usuario_admin) REFERENCES users(id),
	CONSTRAINT usuario_perfil_id_usuario_fkey FOREIGN KEY (id_usuario) REFERENCES users(id)
);

insert into usuario_perfil (id_usuario, id_perfil_usuario, id_usuario_admin) values 
('0190ef4c-2d28-7371-ba3d-2e7c149ac6b0', '402415cc-5b1b-50b0-b2dc-bbcdde013815', '0190ef4c-2d28-7371-ba3d-2e7c149ac6b0'),  --dev
('0190ef5d-796e-7a13-8c73-1d0b1e11d5ba', '6d70c310-d672-56cc-8aaa-32ef2bbb8f20', '0190ef4c-2d28-7371-ba3d-2e7c149ac6b0'), --supervisor
('0190ef6e-0612-7f23-98b0-ed146428556d', 'a6cb735a-e834-5000-9bf5-2d1ea59161d6', '0190ef4c-2d28-7371-ba3d-2e7c149ac6b0'), --(demo) visitante
('0190efad-d52a-7e41-8337-e07796f3860b', '770aed69-191c-5b14-b816-572d02d61564', '0190ef4c-2d28-7371-ba3d-2e7c149ac6b0'), --administrador
('6153aa9d-ea52-4e0e-8b50-bc99571a8b26', 'a6cb735a-e834-5000-9bf5-2d1ea59161d6', '0190ef4c-2d28-7371-ba3d-2e7c149ac6b0'); --visitante

CREATE TYPE user_operation AS ENUM ('edit', 'view');
ALTER TYPE user_operation ADD VALUE 'all';

CREATE TYPE user_permission AS ENUM ('produto', 'empresa', 'usuario', 'pedido', 'contato', 'compra', 'financeiro');

CREATE TABLE if not exists permissao (
	operation user_operation,
	permissao user_permission ,
	id_usuario varchar(40) NOT NULL references users(id)
);


-- --permissoes do usuario dev
-- insert into permissao (operation, permissao, id_perfil) 
-- select
-- 	default_operation.id, 
-- 	default_permission.id, 
-- 	'402415cc-5b1b-50b0-b2dc-bbcdde013815' --dev
-- from 
-- 	default_operation, default_permission;


-- --permissoes do usuario supervisor
-- insert into permissao (operation, permissao, id_perfil) 
-- select
-- 	default_operation.id, 
-- 	default_permission.id, 
-- 	'6d70c310-d672-56cc-8aaa-32ef2bbb8f20' --supervisor
-- from 
-- 	default_operation, default_permission;

-- --permissoes do usuario administrador
-- insert into permissao (operation, permissao, id_perfil) 
-- select
-- 	default_operation.id, 
-- 	default_permission.id, 
-- 	'770aed69-191c-5b14-b816-572d02d61564' --administrador
-- from 
-- 	default_operation, default_permission
-- 	;

-- --permissoes do usuario visitante
-- insert into permissao (operation, permissao, id_perfil) 
-- select
-- 	default_operation.id, 
-- 	default_permission.id, 
-- 	'a6cb735a-e834-5000-9bf5-2d1ea59161d6' --visitante
-- from 
-- 	default_operation, default_permission
-- 	;

-- --permissoes do usuario comum
-- insert into permissao (operation, permissao, id_perfil) 
-- select
-- 	default_operation.id, 
-- 	default_permission.id, 
-- 	'edbde215-0f9d-5e02-a252-1a6290568fea' --user
-- from 
-- 	default_operation, default_permission
-- 	where default_permission.id in ('VIEW', 'CREATE', 'UPDATE')
-- 	;




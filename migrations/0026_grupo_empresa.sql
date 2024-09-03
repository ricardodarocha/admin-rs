-- Add migration script here
CREATE TABLE grupo_produto_empresa (
	id_empresa varchar(40) NOT NULL,
	id_grupo_produto varchar(40) NOT NULL,
	ativo bool NOT NULL DEFAULT true,
	CONSTRAINT grupo_produto_empresa_id_empresa_fkey FOREIGN KEY (id_empresa) REFERENCES empresa(id),
	CONSTRAINT grupo_produto_empresa_id_grupo_produto_fkey FOREIGN KEY (id_grupo_produto) REFERENCES grupo_produto(id)
);

CREATE TABLE grupo_pessoa_empresa (
	id_empresa varchar(40) NOT NULL,
	id_grupo_pessoa varchar(40) NOT NULL,
	ativo bool NOT NULL DEFAULT true,
	CONSTRAINT grupo_pessoa_empresa_id_empresa_fkey FOREIGN KEY (id_empresa) REFERENCES empresa(id),
	CONSTRAINT grupo_pessoa_empresa_id_grupo_pessoa_fkey FOREIGN KEY (id_grupo_pessoa) REFERENCES grupo_pessoa(id)
);
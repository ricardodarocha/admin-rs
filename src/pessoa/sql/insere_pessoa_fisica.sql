insert into pessoa (
	id,   
	razao_social,     
	nome,                
	id_tipo_pessoa,
	id_identificacao,
	id_status,
	id_email, 
	id_telefone    
) VALUES
(
	:id,
	:razao_social,
	:nome,
	(select id from tipo_pessoa where simbolo = $1), --PF PJ
	(select id from identificacao where descricao = $2),
	(select id from status_pessoa where descricao = $3),
	(select contato.id from contato join tipo_contato ema on ema.id = contato.id_tipo_contato 
        where contato.descricao = :email and ema.nome = 'EMAIL'),
	(select contato.id from contato join tipo_contato tel on tel.id = contato.id_tipo_contato 
        where contato.descricao = :telefone and tel.nome = 'TELEFONE')
);
select 
   	pessoa.id, 
   	pessoa.nome,
   	pessoa.razao_social,
   	tp.nome as tipo_pessoa,
   	id.descricao as identificacao,
   	tid.simbolo as tipo_identificacao,
   	sp.descricao as status,
   	tel.descricao as telefone,
   	ema.descricao  as email
   from pessoa
   join tipo_pessoa tp on tp.id = pessoa.id_tipo_pessoa 
   join identificacao id on id.id = pessoa.id_identificacao 
   join status_pessoa sp on sp.id = pessoa.id_status 
   join tipo_identificacao tid on tid.id = id.id_tipo_identificacao
   left join contato tel on tel.id = id_telefone  
   left join contato ema on ema.id = id_email 
   left join users usr on usr.id = id_user
   order by pessoa.nome;
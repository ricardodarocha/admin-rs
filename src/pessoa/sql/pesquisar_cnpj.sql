 select id.id, id.descricao, nome from identificacao id
 join tipo_identificacao tid on tid.id = id.id_tipo_identificacao
 where descricao = :descricao and nome = 'CNPJ'
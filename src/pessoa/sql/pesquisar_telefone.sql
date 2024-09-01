select contato.id, nome from contato
 join tipo_contato tc on tc.id = contato.id_tipo_contato
 where descricao = :descricao
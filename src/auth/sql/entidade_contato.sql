select c.id, descricao, nome as tipo_contato from contato c join tipo_contato tc on tc.id = c.id_tipo_contato
where descricao = $1
alter table estado add _nome varchar not null default '';

alter table cidade add _nome varchar not null default '';

alter table empresa add segmento varchar not null default '';

INSERT into segmento_pessoa (id, classe, nome) VALUES ('ATACADO', 'bg-info', 'ATACADO'),('VAREJO', 'bg-info', 'VAREJO'),
('LATICINIOS', 'bg-warning', 'LATICÍNIOS'),('CALCADOS', 'bg-warning', 'CALÇADOS'), ('VESTUARIO', 'bg-error', 'VESTUÁRIO');
create table segmento_pessoa (id varchar(40) not null primary key,
nome varchar not null,
CLASSE VARCHAR not null default 'bg_warning');

INSERT INTO SEGMENTO_PESSOA VALUES
('bd53186f-dc70-5380-a8ef-8b7c31386c03','Bares e Restaurantes'), 
('0d711f11-572d-54cd-ab17-3c9a6ebc62fc','Padarias e Cafeterias'), 
('a1296e9a-b2b5-520f-a4f6-7642434699af','Pizzarias'), 
('e0a58947-8bd2-5065-af6b-c55ebabb6f4d','Lanchonetes'),  
('3cc35bf1-3b5e-5c2b-9717-49361622bd29','Docerias'), 
('048e983b-8df1-5674-ae10-f895d4d22181','Sorveterias'), 
('49583441-7f6a-5654-8c70-e13aa427ffc3','Confeitarias'), 
('f8591fe2-3cc5-51fc-bd5d-6fbeb4083f4a','Supermercados'), 
('fc7cb33e-7b8b-5a29-ae61-afc556b70e7e','Minimercados'), 
('613a81e3-744a-57ea-a874-d6e2300fce35','Lojas de conveniência'), 
('268d1e76-896d-5234-84bb-bcaa0dcd6336','Farmácias e Drogarias'), 
('407ac9c7-8221-5a0d-846c-9a37e59c93ca','Pet shops'), 
('bbf600cc-4ed4-5cbc-ad5f-1cba6ba85d4f','Floriculturas'), 
('7b31642b-8626-59a7-a48b-6e1ab16a948b','Lojas de roupas'), 
('4222849a-e07b-57f4-a39a-23570b9972df','Lojas de calçados'), 
('bfeee9d0-04d1-5aec-8750-387072d05df8','Lojas de artigos esportivos'), 
('ec9f1770-8769-5b27-ab22-4fff1ddcddce','Lojas de eletrônicos'), 
('7336aa0c-fb2e-5ec6-a8d9-8bce7a47cc26','Lojas de móveis'), 
('43d99441-214f-588d-a641-4049a4e46691','Empresa de decoração'), 
('c06e75d5-09a3-537a-93a5-b143560a51ad','Papelarias'), 
('b8b8c2d5-cf3d-5c5c-b307-8aadff44206c','Lojas de brinquedos'), 
('4aa7a0b3-0c7b-5e7e-b6bd-99fe809eb2b2','Livrarias'), 
('6e9cb776-cb0f-5021-b699-a36a2513ace5','Lojas de cosméticos'), 
('c57baa27-d1b6-5022-b2f5-19921b54a893','Lojas de suplementos alimentares'), 
('14772e25-2530-56ba-8c42-0a9a55e1ce4d','Distribuidoras de bebidas'), 
('9dffc6f9-4071-5bc0-86ff-85f39781b346','Empórios gourmet'), 
('1aa447d8-827f-59b3-a125-285132fc2e6a','Açougues'), 
('3f984c4d-068e-5095-8579-bea76fb837d4','Peixarias'), 
('bcea4806-c777-5c9d-8d8d-d4f6863b2035','Hortifrutis'), 
('c71a4087-1fc5-521b-b8d2-8a267c86e950','Lojas de produtos naturais'), 
('243199a3-f170-5f5e-86f7-6477f0201daf','Cervejarias, Chachaçarias'), 
('eb7ffddb-7c38-506e-bab2-92eded12fd3e','Vinícolas, Distribuidoras de Vinhos'), 
('e8af90d9-293e-59a3-81dd-b3b2c68ac582','Cooperativas'), 
('3a69448a-2990-54c1-a40f-10f73943fa7b','Hotéis, Pousadas e Resorts'), 
('1d8c560b-9122-5301-90d4-1220d55200c7','Clínicas veterinárias'), 
('75bb8170-9177-5cd3-97b8-39072884b1db','Salões de beleza'), 
('0df9cdd1-9901-584c-ba01-c59722ae0d70','Barbearias'), 
('6b4f012d-c3ed-57c3-b99c-67f0a9afb29b','Estúdios de tatuagem'), 
('9206164b-7675-52a4-a4ff-263b814bbb10','Academias de ginástica'), 
('13963650-bfe1-54ec-8688-5ff0dc3b20f3','Estúdios de bem estar, pilates, yoga'), 
('92f51619-1b70-58f2-a7f9-29e77ba45a44','Centros de estética'), 
('81dde527-cb70-5cfd-b17c-9c28fbb7cee6','Consultórios odontológicos'), 
('db221610-7052-5a05-9e53-99fdab647497','Consultórios médicos'), 
('585f61e6-4333-5295-a6a6-fe2ba9d17f30','Clínicas de fisioterapia'), 
('cec9a201-5626-5a73-874c-aa6ac4d28578','Clínicas de estética'), 
('20cba6f7-44ff-5a11-86c5-88ca6757a406','Empresas de eventos'), 
('2d52aca1-08c7-5dbb-a989-f9d58d809907','Empresas de fotografia'), 
('422cd844-ae14-586c-a08f-834c25a9dea2','Empresas de videografia'), 
('b4f477a8-6b02-5d13-8c63-fa2f8df778b0','Gráficas'), 
('f44a409b-e8cf-582f-b6cc-98a0db5d4576','Produtoras de vídeo'), 
('83c4fc9e-c1d7-55c3-b5c5-9f328042d6c1','Agências de publicidade'), 
('5e55da70-05e1-5719-8035-a148b04167fe','Agências de marketing digital'), 
('1d225bff-088a-525e-8d8e-ebc96a79e735','Lojas de artigos para festas'), 
('a6f2cd39-fa30-51cc-ae21-7b43fa0566bd','Escolas, creches'), 
('a3dd4368-ecec-5adb-9799-e1b72e84289c','Universidades'), 
('9e4bb852-af9a-5f64-b19a-80d20d1e14fb','Autoescolas'), 
('8747d31e-803c-5181-b21b-6d963dbe8d60','Lojas de automóveis, concessionárias e locação'), 
('f290effa-39a4-5b73-ac4f-b6bd9ba8753a','Oficinas mecânicas'), 
('d878046c-9924-5273-a9df-7d181b9aad7e','Autopeças'), 
('44c863a7-1d5e-5335-9a83-459e8797f3b0','Postos de combustíveis'), 
('e627fe75-be82-58e9-8d90-84648df9795c','Lojas de materiais de construção'), 
('26d29656-8f16-5c44-b72d-39a4960985fa','Lojas de ferragens e ou ferramentas'), 
('10643d16-d1b4-561a-bb6c-353244799e95','Lojas de tintas'), 
('9656b3a2-0888-565f-ac22-e792e6b71355','Marcenarias, Serralherias'), 
('15e0f1bc-d046-56ba-b3ad-154abccea30a','Empresas de construção civil'), 
('081eeab2-2cf5-5e83-a762-42d689028004','Imobiliárias'), 
('89a20b84-4c24-5760-add6-22795aa1bdb9','Administradoras de imóveis'), 
('18031b2c-7068-596f-a421-63e6ec260b14','Corretores de imóveis'), 
('8ed4b21e-5111-571d-9eb7-a255bda03fe6','Lojas de materiais elétricos'), 
('4af3cfae-bf07-5f1c-99c0-de4d1680a6a7','Lojas de materiais hidráulicos'), 
('32b0bb17-e2df-565b-8f3f-c63622c4ad52','Lojas de bicicletas'), 
('a784a52f-fe7b-5426-a061-f4c1065758bf','Lojas de motocicletas'), 
('5ef97028-14c4-528c-8548-fe13a1b61076','Empresas de limpeza e conservação'), 
('e034371f-2eb6-5b8b-87c5-1d32525dc895','Serviços de motoboy'), 
('957cc20b-c4dd-56e7-a8dd-ee8fa03ebb44','Transportadoras'), 
('98bbd485-b48a-505a-89c8-cc43a593c59a','Empresas de logística'), 
('8645965b-8594-5160-a25c-ba5b83bb8353','Operadoras de turismo'), 
('16ebc146-7be8-591b-9c54-60149a13e284','Agências de viagens'), 
('1e5c2455-69c0-53d2-9d5f-38e44a5d9879','Empresas de seguros'), 
('081a7e98-d4e6-527f-b178-167dcb092766','Escritórios de contabilidade'), 
('50a729c5-b22a-5b32-b458-cf07f2720db2','Escritórios de advocacia'), 
('da3af43b-73ca-5263-a1b0-c35c32c4c557','Empresas de consultoria empresarial'), 
('53cb18d2-474e-5340-ae67-1df59dc69a39','Empresas de tecnologia'), 
('c9b4a7b6-52a0-5dca-b26d-2c2f81ff74e5','Startups'), 
('f2cd22bf-fb81-5409-9833-906a587e38aa','Empresas de software'),
('e7b771d8-7249-554d-b2ff-55cf8f17a7d5','Empresas de internet e telecomunicações'), 
('80280334-4a30-5bca-b911-a1ba9ac3518c','Empresas de energia solar'), 
('12a8e189-7edf-559a-a4cf-c5a5c77ba8da','Empresas de manutenção predial'), 
('f45a7494-bb88-5609-a6e7-a3dd101d07e4','Empresas de segurança eletrônica'), 
('8749c35b-22ed-50d2-a9f9-4c8cf328d13b','Empresas de coworking'), 
('b58bc0e0-b988-51e0-9df7-ef4f3e92de99','Centros de inovação'), 
('ee91a99c-e40e-52bc-9048-2f75a9e8e481','Incubadoras de startups'), 
('691a5085-5e59-55be-976d-5f96de84d66a','Empresas de e-commerce'), 
('7f08c824-b42d-5534-bfc6-6cc05540048a','Lojas online'), 
('6f0dbabf-f684-5899-a42e-fa98145aac29','Marketplaces'), 
('660dafe5-9ad7-5a83-8d05-e1131eb5f0a1','Distribuidoras de alimentos'), 
('ae274a9b-53e9-525a-9909-212df32fae64','Distribuidoras de cosméticos'), 
('86ea3f04-907e-5410-a6a0-65f85108665f','Distribuidoras de medicamentos'), 
('cddade79-5ee4-5492-85ab-8e71496dd202','Distribuidoras de produtos de limpeza'), 
('63f1881b-bf6d-50c0-b3ba-d34a1d93a8dd','Distribuidoras de produtos para pets'), 
('0ae6d5ca-b772-5a42-9784-75eaabd79d17','Distribuidoras de roupas'), 
('e8de4b58-fc1f-5659-bc50-a2199152194f','Empresas de móveis planejados'), 
('e84ea52e-8cf4-5630-9cc2-51a52471ce0f','Fábricas de móveis'), 
('e7c0dae2-aa2c-5d7e-91d8-f10e3708335b','Indústrias de alimentos'), 
('d5ed83ad-6131-52ad-ab9d-2b2021bdf4b5','Indústrias de bebidas'), 
('01eb055c-48e2-53bf-b7a7-9fe5c620a5e8','Indústrias farmacêuticas'), 
('675bcf3a-64a4-55c0-a3c4-e15449fa3a5b','Indústrias têxteis'), 
('06f1a18e-0877-522c-9644-8b4ab2f958b2','Indústrias de calçados'), 
('4b610ec8-22da-5738-b0a4-5c050563c77b','Indústrias de cosméticos'), 
('004c3f61-8793-576e-b25c-95a8aef87acd','Indústrias de produtos de limpeza'), 
('b3751b00-433d-5fa8-9dcb-bde1ad63472d','Indústrias de plásticos'), 
('722cd3cc-992f-5b06-88e3-ad4ddf278b3c','Indústrias de embalagens'), 
('8a050a6a-ea1e-56a7-b72d-cb3a1b2b536d','Indústrias de metalurgia'), 
('2ba51f33-1b3e-5788-accb-5d55261b7e82','Indústrias químicas'), 
('45ffd039-6197-5ba9-9be4-8b0510d88e75','Indústrias de papel e celulose'), 
('26384ec8-33e3-55d0-b45b-3c5f7418e3d8','Indústrias automotivas'), 
('96561db6-aabb-50eb-bd88-0f408743efda','Indústrias de autopeças'), 
('7fa8c0e4-3e07-5f0a-b298-5f05a04f75cb','Indústrias de eletrônicos'), 
('100add84-b1b3-5498-90e4-c2fe9ebddbce','Indústrias de eletrodomésticos'), 
('428821ac-61d1-5be8-90dd-8b02bc730027','Indústrias de móveis'), 
('64e0b66d-7d74-5a81-a3f2-589d7b25ce92','Indústrias de brinquedos'), 
('0224cf4b-620a-5e53-b189-a214ce9d4230','Indústrias de produtos esportivos'), 
('2e2bc3fd-dbbb-5288-8b91-6135a8c53f73','Indústrias de equipamentos de saúde'), 
('97297f2f-0cac-5e36-acf3-f4ca30b2fa16','Indústrias de materiais de construção'), 
('88527681-1bb5-51d6-98d3-6fe898793c47','Construtoras'), 
('4c9f35c2-f4e3-59e1-9aa9-f3727275f730','Escritórios de engenharia'), 
('4ec9cc0e-9331-58ef-9f9c-c1c34b80d687','Empresas de design de interiores'), 
('db798f21-42a2-53e4-b8d7-610ba9a28f2b','Produtoras de eventos'), 
('ec2cc36e-f147-5920-83c3-c64446941ecc','Empresas de aluguel de equipamentos'), 
('83580ef0-bf46-5098-bde3-477bf06102f3','Empresas de entretenimento'), 
('0b60dd85-1c11-57c0-9252-05d35805ddab','Cinemas'), 
('4e94292d-dbbd-5f52-a088-26185b64f5b5','Teatros'), 
('c03f31d8-ebb4-51da-950d-5f3977251028','Casas de shows'), 
('77190b22-c3c6-57a9-8308-f9e21f376294','Produtoras de espetáculos'), 
('68352284-21dc-577a-91a1-1ccc2ef0d3db','Agências de modelos'), 
('32dd4416-8540-55ce-bd05-a8bf6f5a1579','Centros de convenções'), 
('c0c0c22b-4e9f-53f9-9f5d-96cecce13ebb','Salas de conferências'), 
('a830263a-0e21-5c94-890b-4003b382e590','Centros de exposições'); 

drop table if exists tipo_contato  cascade; 

create table if not exists tipo_contato  (
    id  VARCHAR(40) not null PRIMARY key,
    nome VARCHAR not null --telefone, email, 
    );
   
insert into tipo_contato (id, nome) values 
  ('a0a66748-1a9e-5b95-a418-7270bf5caba8', 'TELEFONE'),
  ('4920966b-d6fb-51a2-8218-7ede4eb28d0c', 'EMAIL'),
  ('bb7a936b-faf3-5814-ac8c-1412172019fb', 'WHATSAPP'),
  ('5f348c82-c947-5c25-98a6-7324c5081635', 'INSTAGRAM'),
  ('36273362-32b2-59f7-b0f4-e3805e3c6148', 'SKYPE'),
  ('0e09a04c-b4e6-5a1d-a12b-f34559323a8e', 'TELEGRAM'),
  ('37245b38-f5d6-5f1d-bad9-5cd481fa93be', 'DISCORD'),
  ('INDEFINIDO', 'INDEFINIDO') ;

create table if not exists contato  (
    id  VARCHAR(40) not null PRIMARY key,
    id_tipo_contato VARCHAR(40) not null references tipo_contato(id),
    descricao VARCHAR 
    );


--fixa alguns contatos 
insert into contato values (
'f5cde193-1f51-51ae-9b11-d41930704ab0',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'admin@acme.com'), (
'f5cde193-1f51-51ae-9b11-d41930704ab3',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'sac.venturi@gmail.com'), (
'0f81ca8e-2981-5f88-aa95-41c8cbc22a68',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'ricardodarocha@outlook.com'), (
'6153aa9d-ea52-4e0e-8b50-bc99571a8b26',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'admin@acme.com'), (
'01911a44-5f65-75c6-9c56-1ce2621f1508',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'demo@sistema.com'), (
'01911a44-5f65-7f4f-9de1-cefc9b4b78bb',	'4920966b-d6fb-51a2-8218-7ede4eb28d0c',	'teste@sistema.com') (
'INDEFINIDO',	'INDEFINIDO',	'INDEFINIDO')
    ;
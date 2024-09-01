CREATE TABLE public.icones (
	id varchar(40) NOT NULL,
	nome varchar(40) NULL,
	classe varchar(40) NULL,
	CONSTRAINT icones_pkey PRIMARY KEY (id)
);

INSERT INTO public.icones (id,nome,classe) VALUES 
('661103','fas fa-fone','Telefone')
,('378437','fas fa-envelope','Envelope')
,('910956','fas fa-file','Arquivo')
,('587780','fas fa-Print','Impressora')
,('861965','fas fa-Folder','Pasta')
,('743094','fas fa-folder-open','Abrir Pasta')
,('405571','fas fa-city','Cidade')
,('541660','fas fa-landmark','Banco')
,('128674','fas fa-chart-simple','Gráfico de barras')
,('267996','fas fa-calendar','Calendário')
;
INSERT INTO public.icones (id,nome,classe) VALUES 
('114860','fas fa-pencil','Lápis')
,('475177','fas fa-copy','Copiar')
,('959693','fas fa-paste','Colar')
,('645136','fas fa-cut','Recortar')
,('964565','fas fa-table','Tabela')
,('782318','fas fa-building','Edifício')
,('377940','fas fa-network-wired','Rede')
,('881117','fas fa-globo','Globo')
,('283122','fas fa-tag','Etiqueta')
,('250476','fas fa-address-card','Cartão')
;
INSERT INTO public.icones (id,nome,classe) VALUES 
('859054','fas fa-tags','Várias etiquetas')
,('301643','fas fa-wallet','Carteira')
,('206146','fas fa-calculator','Calculadora')
,('315923','fas fa-certificate','Certificado')
,('824656','fas fa-file-spreadsheet','Planilha')
,('678716','fas fa-glasses','Óculos')
,('984266','fas fa-industry','Fábrica')
,('610808','fas fa-clipboard','Prancheta')
,('824953','fas fa-table-layout','Layout')
,('365400','fas fa-table-columns','Tabela com duas colunas')
;
INSERT INTO public.icones (id,nome,classe) VALUES 
('388423','fas fa-scissors','Tesoura')
,('938332','fas fa-presentation-screen','Projetor')
,('652568','fas fa-notebook','Caderneta')
,('911926','fas fa-lamp-desk','Luminária')
,('195717','fas fa-furniture','Mobiliário')
,('252922','fas fa-furniture','Mobiliário')
,('537387','fas fa-mug-saucer','Caneca')
,('823616','fas fa-briefcase','Maleta')
,('907320','fas fa-address-book','Livro de Endereços')
,('563458','fas fa-address-card','Cartão de Endereço')
;
INSERT INTO public.icones (id,nome,classe) VALUES 
('553890','fas fa-adjust','Ajustar')
,('157496','fas fa-air-freshener','Purificador de Ar')
,('607820','fas fa-align-center','Alinhar ao Centro')
,('541536','fas fa-align-justify','Justificar Alinhamento')
,('503322','fas fa-align-left','Alinhar à Esquerda')
,('980594','fas fa-align-right','Alinhar à Direita')
,('834718','fas fa-allergies','Alergias')
,('347695','fas fa-ambulance','Ambulância')
,('485668','fas fa-anchor','Âncora')
,('672265','fas fa-angle-double-down','Seta Dupla para Baixo')
;
INSERT INTO public.icones (id,nome,classe) VALUES 
('863058','fas fa-angle-double-left','Seta Dupla para a Esquerda')
,('971741','fas fa-angle-double-right','Seta Dupla para a Direita')
,('908221','fas fa-angle-double-up','Seta Dupla para Cima')
,('367849','fas fa-angle-down','Seta para Baixo')
,('677165','fas fa-angle-left','Seta para a Esquerda')
,('474598','fas fa-angle-right','Seta para a Direita')
,('699180','fas fa-angle-up','Seta para Cima')
,('102066','fas fa-archive','Arquivo')
,('898667','fas fa-archway','Arco')
,('687715','fas fa-arrow-alt-circle-down','Círculo com Seta para Baixo')
;
INSERT INTO public.icones (id,nome,classe) VALUES 
('494973','fas fa-arrow-alt-circle-left','Círculo com Seta para a Esquerda')
,('479439','fas fa-arrow-alt-circle-right','Círculo com Seta para a Direita')
,('548570','fas fa-arrow-alt-circle-up','Círculo com Seta para Cima')
,('129164','fas fa-arrow-circle-down','Círculo com Seta para Baixo')
,('856716','fas fa-arrow-circle-left','Círculo com Seta para a Esquerda')
,('138217','fas fa-arrow-circle-right','Círculo com Seta para a Direita')
,('761818','fas fa-arrow-circle-up','Círculo com Seta para Cima')
,('506968','fas fa-arrow-down','Seta para Baixo')
,('972049','fas fa-arrow-left','Seta para a Esquerda')
,('393995','fas fa-arrow-right','Seta para a Direita')
;
INSERT INTO public.icones (id,nome,classe) VALUES 
('915072','fas fa-arrow-up','Seta para Cima')
,('901995','fas fa-arrows-alt','Setas em Todas as Direções')
,('795244','fas fa-arrows-alt-h','Setas Horizontais')
,('969804','fas fa-arrows-alt-v','Setas Verticais')
,('997122','fas fa-assistive-listening-systems','Sistemas de Audição Assistiva')
,('998404','fas fa-asterisk','Asterisco')
,('229850','fas fa-at','Arroba')
;

--Cria uma lista de menus que exigem permissão do usuário
create table menus (
  id varchar(40) not null primary key,
  sistema varchar(40),
  seq serial,
  caminho varchar(40),
  classe varchar(40),
  titulo varchar(40),
  descricao varchar(40),
  contexto varchar(40) not null default 'main'  
);

-- Cria alguns menus do sistema
insert into menus (id, caminho, classe, titulo, descricao
) values (
floor(random() * (999999-099999+1) + 099999)::varchar,
'/dashboard',
'fas fa-briefcase',
'Dashboard',
'Abre o dashboard do usuário');

create table menu_usuario (
id_usuario varchar(40) not null references users(id),
id_menu varchar(40) not null references menus(id),
constraint pkmenu_usuario PRIMARY KEY(id_usuario, id_menu)
);
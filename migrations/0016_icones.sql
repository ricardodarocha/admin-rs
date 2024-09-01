-- Adiciona suporte aos ícones font-awesome ao sistema
create table icones (
    id varchar(40) not null primary key,
    nome varchar(40),
    classe varchar(40)
);

--insere alguns ícones padrão                
insert into icones values
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-briefcase', 'Maleta'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-address-book', 'Livro de Endereços'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-address-card', 'Cartão de Endereço'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-adjust', 'Ajustar'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-air-freshener', 'Purificador de Ar'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-align-center', 'Alinhar ao Centro'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-align-justify', 'Justificar Alinhamento'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-align-left', 'Alinhar à Esquerda'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-align-right', 'Alinhar à Direita'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-allergies', 'Alergias'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-ambulance', 'Ambulância'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-anchor', 'Âncora'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-angle-double-down', 'Seta Dupla para Baixo'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-angle-double-left', 'Seta Dupla para a Esquerda'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-angle-double-right', 'Seta Dupla para a Direita'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-angle-double-up', 'Seta Dupla para Cima'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-angle-down', 'Seta para Baixo'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-angle-left', 'Seta para a Esquerda'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-angle-right', 'Seta para a Direita'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-angle-up', 'Seta para Cima'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-archive', 'Arquivo'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-archway', 'Arco'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-alt-circle-down', 'Círculo com Seta para Baixo'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-alt-circle-left', 'Círculo com Seta para a Esquerda'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-alt-circle-right', 'Círculo com Seta para a Direita'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-alt-circle-up', 'Círculo com Seta para Cima'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-circle-down', 'Círculo com Seta para Baixo'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-circle-left', 'Círculo com Seta para a Esquerda'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-circle-right', 'Círculo com Seta para a Direita'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-circle-up', 'Círculo com Seta para Cima'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-down', 'Seta para Baixo'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-left', 'Seta para a Esquerda'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-right', 'Seta para a Direita'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrow-up', 'Seta para Cima'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrows-alt', 'Setas em Todas as Direções'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrows-alt-h', 'Setas Horizontais'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-arrows-alt-v', 'Setas Verticais'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-assistive-listening-systems', 'Sistemas de Audição Assistiva'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-asterisk', 'Asterisco'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-at', 'Arroba'),                
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-fone', 'Telefone'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-envelope', 'Envelope'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-file', 'Arquivo'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-Print', 'Impressora'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-Folder', 'Pasta'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-folder-open', 'Abrir Pasta'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-city', 'Cidade'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-landmark', 'Banco'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-chart-simple', 'Gráfico de barras'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-calendar', 'Calendário'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-pencil', 'Lápis'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-copy', 'Copiar'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-paste', 'Colar'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-cut', 'Recortar'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-table', 'Tabela'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-building', 'Edifício'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-network-wired', 'Rede'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-globo', 'Globo'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-tag', 'Etiqueta'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-address-card', 'Cartão'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-tags', 'Várias etiquetas'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-wallet', 'Carteira'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-calculator', 'Calculadora'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-certificate', 'Certificado'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-file-spreadsheet', 'Planilha'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-glasses', 'Óculos'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-industry', 'Fábrica'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-clipboard', 'Prancheta'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-table-layout', 'Layout'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-table-columns', 'Tabela com duas colunas'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-scissors', 'Tesoura'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-presentation-screen', 'Projetor'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-notebook', 'Caderneta'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-lamp-desk', 'Luminária'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-furniture', 'Mobiliário'),
(floor(random() * (999999-099999+1) + 099999)::varchar,'fas fa-mug-saucer', 'Caneca')
;
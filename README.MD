# API

Template SAAS com Rust e Minijinja
Visual: Daisy Ui
Backend: Actix web

![Previa](https://github.com/ricardodarocha/admin-rs/blob/main/preview.gif?raw=true)

## Como testar

clone este regositorio em sua pasta, em seguida instale as dependências com `npm install`
As dependências JS incluem Chart.js, Daisy UI e Tailwind

Em seguida rode o comando `watch` para rodar

`git clone https://github.com/ricardodarocha/admin-rs.git meu-projeto-saas`
`npm install`
`watch`

## Como usar o template

Acesse o github e procure o botão "Use this template" > "Create a repository", este processo te guiará a criar o próprio repositóio a partir deste template

## Banco de dados

O banco de dados do exemplo é sqlite, portanto simples de testar, proém também é possível configurar o postgre
O motor de banco de dados é sqlx

## Utilitário de banco de dados

Para rodar o migrations, isntale o sqlx-cli

`cargo install sqlx-cli`
`sqlx migrate run`

Para criar novas migrations é possível rodar sqlx migrate add `<nome-tabela>`, configurar o script `CREATE TABLE` e rodar novamente `migrate run`

![Previsualização do site](https://github.com/ricardodarocha/api/blob/templates/preview.GIF)

## Arquitetura da aplicação

```Php
src/
├── admin/                # Acessa as urls com permissão restrita a administrador 
├── application/          # Camada de aplicação - lógica de casos de uso, 
│   │                     # .. interage com a camada de domínio
│   ├── operacoes/        # Comandos de criação, atualização e deleção
│   │   └── create_order.rs
│   │   └── update_order.rs
│   └── consulta/          # Consultas (ex.: buscar por ID, listar todos)
│       └── get_order.rs
│       └── list_orders.rs
├── api/                  # Disponibiliza a api para terceiros
├── config/               # Configuração da aplicação, como leitura de arquivos de ambiente
│   └── database.rs
├── core/                 # Camada de domínio - modelos, repositórios e lógica de negócio
│   ├── entities/         # Modelos principais da aplicação (entidades de domínio)
│   │   └── pedido.rs     # Ex.: struct Pedido, com métodos e lógica do modelo
│   └── traits/           # Interfaces para operações de leitura e escrita
│       └── consulta_bd.rs
│       └── escreve_bd.rs
├── infrastructure/       # Camada de infraestrutura - 
│   │                     # implementações concretas para repositórios, conexão com DB, etc.
│   ├── database/         # Módulo de acesso ao banco de dados
│   │   └── pedido_repo.rs # Implementação de repositório usando sqlx
│   └── web/              # Adaptadores web, configurando rotas e controladores
│       └── routes.rs     # Define rotas e utiliza casos de uso da camada de aplicação
├── main.rs               # Ponto de entrada da aplicação
└── lib.rs                # Importa módulos e inicializa a estrutura

```

Se precisar atualizar o Rust use o comando `rustup update stable`

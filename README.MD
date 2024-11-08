# API

Template SAAS com Rust e Minijinja
Visual: Daisy Ui
Backend: Actix web

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

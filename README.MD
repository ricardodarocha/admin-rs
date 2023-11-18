# Lançada a versão Rocket 0.5 stable

Se você quiser construir por si mesmo esta api com a ajuda do ChatGpt-4 aqui vai um exemplo inicial,
porém eu recomendo que você clone este repositório e mude apenas as rotas porque pode haver uma dificuldade para configurar o Postres ou Sqlite no início
Veja o arquivo rocket.toml e o módulo db.rs

*Pegue um atalho, clone este repositorio e altere o banco de dados e as rotas*

```rust
#[macro_use] extern crate rocket;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
```

Exemplo do setup ao término do projeto

```rust
pub mod reg;
pub mod auth;
#[macro_use] extern crate rocket;

pub mod mate;
pub mod prod;
pub mod db;

use mate::*;
use prod::*;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

// use rocket::serde::{Serialize, Deserialize, json::{Json, Value}};
use rocket_db_pools::Database;
use crate::db::DbMeuBanco;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(db::DbMeuBanco::init())
        .mount("/", routes![ 
            hello,  
            //seus módulos
            suas_rotas_aqui,
            ]
    
    )
        .launch()
        .await;
}


```

## Configuração do banco de dados

ver migrations, db.rs e rocket.toml

```rust
// src/db/mod.rs
use sqlx::{SqlitePool, Pool, Sqlite};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("sqlite_meu_banco")]
pub struct DbMeuBanco(sqlx::SqlitePool);
use rocket::fairing::AdHoc;

pub async fn init_db_pool(database_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePool::connect(database_url).await
}
```

Observar como o banco de dados deve ser passado nas rotas assíncronas

```rust
#[post("/prod/json", format = "json", data = "<product>")]
pub async fn newprod(mut db: Connection<DbMeuBanco>, product: Json<Product>) -> Result<Json<Product>, status::Custom<String>> {
    sqlx::query_as::<_, Product>("INSERT INTO PRODUTO (name, price, tipo) values (?/*name*/, ?/*price*/, 'produto') returning id, name, price, tipo")
        .bind(&product.name)
        .bind(product.price)
        .fetch_one(&mut **db)
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}
```

Este código usa o atributo #[get("/hello")] para declarar uma rota para solicitações HTTP GET no caminho /hello. A função hello retorna uma string estática. A função rocket constrói e configura o aplicativo Rocket, montando a rota hello. O atributo #[launch] marca-o como o ponto de entrada para o aplicativo. Para executar isso, adicione rocket = "0.5" ao seu Cargo.toml em [dependências].

Para adicionar uma rota que lista produtos no seu aplicativo Rocket v0.5, você primeiro precisa definir uma estrutura para os produtos e uma função que retorna uma lista desses produtos. Aqui está um exemplo simples

```rust
#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Product {
    id: i32,
    name: String,
    price: f32,
}

#[get("/products")]
fn list_products() -> Json<Vec<Product>> {
    let products = vec![
        Product { id: 1, name: "Produto A".to_string(), price: 10.99 },
        Product { id: 2, name: "Produto B".to_string(), price: 20.99 },
    ];

    Json(products)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![list_products])
}

```

Neste código:

Product é uma estrutura que representa um produto, com campos como id, name e price.
list_products é uma função que cria uma lista de produtos e a retorna como JSON.
A rota /products é montada para servir a lista de produtos.
Veja que as dependências devem ser adicionadas ao seu toml:

[dependencies]
rocket = "0.5"
serde = { version = "1.0", features = ["derive"] }

Para adicionar uma rota que recupera um único produto pelo seu ID em um aplicativo Rocket v0.5, você precisará criar uma rota que aceita um parâmetro na URL. Vou modificar o exemplo anterior para incluir essa nova rota:

```rust
#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Product {
    id: i32,
    name: String,
    price: f32,
}

#[get("/products")]
fn list_products() -> Json<Vec<Product>> {
    let products = vec![
        Product { id: 1, name: "Produto A".to_string(), price: 10.99 },
        Product { id: 2, name: "Produto B".to_string(), price: 20.99 },
    ];

    Json(products)
}

#[get("/product/<id>")]
fn get_product(id: i32) -> Option<Json<Product>> {
    let products = vec![
        Product { id: 1, name: "Produto A".to_string(), price: 10.99 },
        Product { id: 2, name: "Produto B".to_string(), price: 20.99 },
    ];

    products.into_iter().find(|p| p.id == id).map(Json)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![list_products, get_product])
}
```

Para adicionar uma rota que permite cadastrar um novo produto no seu aplicativo Rocket v0.5, você precisa criar uma rota que aceite dados POST.
Aqui a gente também começa a trabalhar com serde para manipular json, além da camada serde disponibilizada pelo Rocket;


`serde = { version = "1.0", features = ["derive"] }`

voce tambem pode usar  o comando `cargo add serde --features derive` se preferir

```rust
#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Product {
    id: i32,
    name: String,
    price: f32,
}

#[get("/products")]
fn list_products() -> Json<Vec<Product>> {
    // Exemplo com produtos estáticos
    Json(vec![
        Product { id: 1, name: "Produto A".to_string(), price: 10.99 },
        Product { id: 2, name: "Produto B".to_string(), price: 20.99 },
    ])
}

#[get("/product/<id>")]
fn get_product(id: i32) -> Option<Json<Product>> {
    // Exemplo com produtos estáticos
    let products = vec![
        Product { id: 1, name: "Produto A".to_string(), price: 10.99 },
        Product { id: 2, name: "Produto B".to_string(), price: 20.99 },
    ];

    products.into_iter().find(|p| p.id == id).map(Json)
}

#[post("/product", format = "json", data = "<product>")]
fn new_product(product: Json<Product>) -> Json<Product> {
    // Aqui você pode adicionar lógica para salvar o produto
    product
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![list_products, get_product, new_product])
}
```

Product agora também implementa Deserialize para permitir a desserialização dos dados JSON recebidos.
A função new_product aceita dados JSON enviados numa requisição POST, desserializa-os para um Product, e então retorna o produto. Em um caso real, você adicionaria lógica para salvar o produto em um banco de dados ou outro repositório de dados.
Agora, você tem uma rota POST /product que aceita dados JSON para criar um novo produto. Note que neste exemplo, a funcionalidade de armazenamento persistente não está implementada, então o produto recebido simplesmente é retornado como resposta. Em um aplicativo real, você precisaria implementar a lógica de persistência de dados.

Para adicionar uma rota que permite a atualização parcial de um produto (conhecida como atualização "patch") no seu aplicativo Rocket v0.5, você pode criar uma rota que aceite uma requisição PATCH. Essa rota deverá receber o ID do produto e os campos a serem atualizados.

Vamos modificar o exemplo anterior para incluir essa nova rota. Para simplificar, vou simular a atualização sem persistência em um banco de dados:

```rust
#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::{Json, Value}};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Product {
    id: i32,
    name: String,
    price: f32,
}

// Simulando um banco de dados com um HashMap
type Db = HashMap<i32, Product>;

#[patch("/product/<id>", format = "json", data = "<product_patch>")]
fn update_product(id: i32, product_patch: Json<Value>, db: &rocket::State<Db>) -> Option<Json<Product>> {
    let mut db = db.lock().unwrap();
    if let Some(product) = db.get_mut(&id) {
        if let Some(name) = product_patch.get("name").and_then(Value::as_str) {
            product.name = name.to_string();
        }
        if let Some(price) = product_patch.get("price").and_then(Value::as_f64) {
            product.price = price as f32;
        }
        return Some(Json(product.clone()));
    }
    None
}

#[launch]
fn rocket() -> _ {
    let db: Db = vec![
        (1, Product { id: 1, name: "Produto A".to_string(), price: 10.99 }),
        (2, Product { id: 2, name: "Produto B".to_string(), price: 20.99 }),
    ].into_iter().collect();

    rocket::build()
        .manage(Mutex::new(db))
        .mount("/", routes![update_product])
}

```

A função update_product recebe o ID do produto e um JSON com os campos a serem atualizados.
Usamos um HashMap como um banco de dados simulado para armazenar produtos. Em um aplicativo real, você usaria um banco de dados de verdade.
O JSON recebido é usado para atualizar os campos name e price do produto, se eles estiverem presentes na requisição.
A rota PATCH /product/<id> agora permite atualizar parcialmente um produto existente.

Para integrar um banco de dados SQLite usando SQLx em um aplicativo Rocket v0.5, você precisa seguir alguns passos. Vamos começar adicionando as dependências necessárias no seu Cargo.toml:

``` 
sqlx = { version = "0.7.2", features = ["runtime-tokio-rustls", "sqlite"] }
tokio = { version = "1", features = ["full"] }
```
```rust
#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Product {
    id: i32,
    name: String,
    price: f32,
}

#[get("/products")]
async fn list_products(db: &rocket::State<SqlitePool>) -> Result<Json<Vec<Product>>, sqlx::Error> {
    let products = sqlx::query_as!(Product, "SELECT * FROM products")
        .fetch_all(db.inner()).await?;

    Ok(Json(products))
}

// Outras rotas aqui...

#[launch]
async fn rocket() -> _ {
    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite:meu_banco.db").await
        .expect("Erro ao conectar ao banco de dados");

    // Inicializando o banco de dados (Opcional)
    sqlx::query("CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY, name TEXT NOT NULL, price REAL NOT NULL, tipo VARCHAR(10))")
        .execute(&db_pool).await
        .expect("Erro ao criar a tabela prod");

    rocket::build().manage(db_pool).mount("/", routes![list_products])
}
```

Para iniciar o banco de dados adicione uma última crate 

```
cargo install sqlx-cli
cargo sqlx prepare
```

Criando e Manipulando Dados:

Para criar, atualizar e manipular dados, você usaria comandos SQLx semelhantes dentro das funções correspondentes a cada rota. Por exemplo, para adicionar um produto, você pode usar `sqlx::query!("INSERT INTO products (name, price) VALUES (?, ?)", new_product.name, new_product.price).execute(&*db).await.`

Certifique-se de adaptar cada rota para operar de forma assíncrona e interagir com o banco de dados.
Trate os erros adequadamente em suas rotas.
Você pode querer adicionar lógica para iniciar e gerenciar transações onde for necessário.
Este exemplo estabelece uma base para um aplicativo Rocket com SQLx e SQLite, incluindo operações CRUD básicas. Você precisará expandir e modificar este código para atender às necessidades específicas do seu aplicativo e garantir que todas as operações do banco de dados sejam realizadas corretamente.

Por fim nós separamos  em módulos para deixar tudo organizado

1. Estrutura de Diretórios
Suponha a seguinte estrutura de diretórios:

lua
Copy code
src/
|-- db/
|   |-- mod.rs
|-- routes/
|   |-- mod.rs
|-- main.rs

2. Módulo de Banco de Dados (db)
No arquivo src/db/mod.rs, você configurará o pool de conexão e as funções relacionadas ao banco de dados.

```rust
// src/db/mod.rs
use sqlx::{SqlitePool, Pool, Sqlite};
use rocket::fairing::AdHoc;

pub async fn init_db_pool(database_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePool::connect(database_url).await
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database Stage", |rocket| async {
        let database_url = rocket.figment().extract_inner::<String>("database.url")
            .unwrap_or_else(|_| "sqlite:meu_banco.db".to_string());

        let db_pool = match init_db_pool(&database_url).await {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Database connection failed: {}", e);
            }
        };

        rocket.manage(db_pool)
    })
}

```

3. Módulo de Rotas (routes)
No arquivo src/routes/mod.rs, você definirá as rotas do seu aplicativo.

```rust
// src/routes/mod.rs
use rocket::serde::{json::Json, Serialize, Deserialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    id: i32,
    name: String,
    price: f32,
}

#[get("/products")]
pub async fn list_products(db: &rocket::State<SqlitePool>) -> Result<Json<Vec<Product>>, sqlx::Error> {
    let products = sqlx::query_as!(Product, "SELECT * FROM products")
        .fetch_all(db.inner()).await?;

    Ok(Json(products))
}

// Outras rotas aqui...
```

```rust
// src/main.rs
#[macro_use] extern crate rocket;

mod db;
mod routes;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(db::stage())
        .mount("/", routes![routes::list_products])
        .launch()
        .await;
}

```
A função db::init_db_pool inicia o pool de conexões do SQLite, e db::stage é um 'stage' Rocket para inicializar e gerenciar a conexão com o banco de dados. As rotas são definidas no módulo routes.
No main.rs, o aplicativo é configurado e iniciado com os módulos db e routes.
Certifique-se de que as dependências corretas estão no seu Cargo.toml e ajuste o código conforme necessário para atender às necessidades específicas do seu aplicativo. O comando final para compilar, procurando por erros e recebendo sugestões de ajuste

cargo check
cargo clippy
cargo build --release
cargo run --release

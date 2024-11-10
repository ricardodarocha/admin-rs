use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use crate::infra::result::Result;

/// Define um contrato de Repositório para qualquer entidade T
///    - tipo Entidade -> O próprio Modelo com ID no banco de dados
///    - tipo CreatePayload -> Um model qualquer que represente um formulário com os dados a serem inseridos
///    - tipo UpdatePayload -> Um model qualquer que represente um formulário com os dados a serem atualizados
///    - tipo Id -> Um tipo que represente o ID no banco de dados, pode ser i32, String, UUID etc
///    - Filter -> Um tipo de parametros de consulta, query paramns, filtros etc
#[async_trait]
pub trait Repository {
    type Entity;
    type CreatePayload;
    type UpdatePayload;
    type Id;
    // type Filter;

    async fn create(pool: &Pool<Sqlite>, payload: &Self::CreatePayload) -> Result<Self::Entity>;
    async fn update(pool: &Pool<Sqlite>, id: Self::Id, payload: &Self::UpdatePayload) -> Result<Self::Entity>;
    async fn delete(pool: &Pool<Sqlite>, id: Self::Id) -> Result<()>;
    async fn get_by_id(pool: &Pool<Sqlite>, id: Self::Id) -> Result<Self::Entity>;
                
}

/// Desassociando a camada de leitura da camada descrita é possível implementar diversas consultas
/// individuais usando o filtro que quiser
/// `impl Consulta for FiltroPeriodo { fn get(, filtro: FiltroPeriodo) -> Pedido {};} `
#[async_trait]
pub trait ConsultaBd { 
    type Entity;
    
    async fn get<'a>(pool: &'a Pool<Sqlite>, filtro: &'a Self) -> Result<Vec<Self::Entity>>
        where
        Self: Sized;
    }

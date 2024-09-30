use crate::infra::result::Result;
use crate::infra::uuid::UuidValue;

pub trait Repository {
    fn inserir<M>(model: M) -> Result<Self>
    where
        Self: Sized;

    fn abrir(id: UuidValue) -> Result<Self>
    where
        Self: Sized;

    fn atualizar<M>(id: UuidValue, model: M) -> Result<Self>
    where
        Self: Sized;

    fn excluir(id: UuidValue) -> Result<()>
    where
        Self: Sized;
        
    fn filtrar<F>(filtro: F) -> Result<Vec<Self>>
    where
        Self: Sized;
}

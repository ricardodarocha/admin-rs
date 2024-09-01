pub use crate::infra::error::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

use serde::Serialize;
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TipoToast {
    Success,
    Info,
    Warn,
    Error   
}

#[derive(Serialize)]
pub struct Toast {
    #[serde(rename="type")]
    pub tipo: TipoToast,
    pub icon: String,
    pub text: String,

}

use crate::infra::error::Error;

impl From<Error> for Toast {
    fn from(value: Error) -> Self {

        let text = format!("{:?}", value); 

        Toast {
            tipo: TipoToast::Error,
            icon: "bi-x-circle-fill".to_owned(),
            text,
        }
    }
}
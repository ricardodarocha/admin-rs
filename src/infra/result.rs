//use std::default;
use serde::{Serialize, Deserialize};
pub use crate::infra::error::Error;
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

/// Empacota um array de itens, exibindo os metadados da requisição
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Pacote<T> {
    pub info: Vec<String>,
    pub page: u32, //qual a página atual
    pub capacity: u32, //quantos registros cabem na página
    pub count: u32, //quantos registros estão sendo retornados agora
    pub has_next: bool,
    pub warn: Vec<String>,
    pub erros: Vec<String>,
    pub dados: Vec<T>,
    pub status: u32,
    pub mensagem: Option<String>,
}

impl<T> Pacote<T> {
    /// ℹ Adiciona um aviso à lista de `warn`
    pub fn aviso(mut self, aviso: &str) -> Self {
        self.warn.push(aviso.to_string());
        self
    }
    /// ⚠ Adiciona um warn à lista de `warn`
    pub fn alerta(mut self, aviso: &str) -> Self {
        self.warn.push(aviso.to_string());
        self
    }

    /// ❌ Adiciona um erro à lista de `erros`
    pub fn erro(mut self, erro: &str) -> Self {
        self.erros.push(erro.to_string());
        self
    }

    /// Adiciona um item ao campo `dados`
    pub fn dado(mut self, dado: T) -> Self {
        self.dados.push(dado);
        self.count = self.dados.len() as u32; // Atualiza o count conforme o número de itens em `dados`
        self
    }

    /// Define a página atual
    pub fn pagina(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    /// Define a capacidade da página
    pub fn capacidade(mut self, capacity: u32) -> Self {
        self.capacity = capacity;
        self
    }

    /// Define o status da resposta
    pub fn status(mut self, status: u32) -> Self {
        self.status = status;
        self
    }

    /// Define a mensagem opcional da resposta
    pub fn mensagem(mut self, message: &str) -> Self {
        self.mensagem = Some(message.to_string());
        self
    }

    /// Define se há uma próxima página ou não
    pub fn tem_proximo(mut self, has_next: bool) -> Self {
        self.has_next = has_next;
        self
    }
}

impl<T: Default> From<Vec<T>> for Pacote<T> {
    fn from(vec: Vec<T>) -> Self {
        Self { 
            count: vec.len() as u32,
            dados: vec,
            .. Default::default()
         }
    }
}
   
impl<T: Default> From<sqlx::Error> for Pacote<T> {
    fn from(err: sqlx::Error) -> Self {
        Self { 
            count: 0,
            dados: vec!(),
            erros: vec!(format!("{}", err)),
            .. Default::default()
         }
    }
}

impl<T: Default> From<Result<Vec<T>>> for Pacote<T> {
    fn from(result: Result<Vec<T>>) -> Self {
        match result {
            Ok(vec) => {
                // Se o resultado for Ok, preenche o `dados` com o `Vec<T>` e o `count` com o tamanho do vetor
                Self {
                    count: vec.len() as u32,
                    dados: vec,
                    ..Default::default()
                }
            }
            Err(e) => {
                // Se o resultado for Err, deixa `dados` vazio e preenche `erros` com o erro
                Self {
                    count: 0,
                    dados: vec![],
                    erros: vec![format!("{:?}", e)], // captura a string do erro
                    ..Default::default()
                }
            }
        }
    }
}


impl<T: Default> From<Option<T>> for Pacote<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(data) => {
                // Se for Some, coloca o valor no campo `dados` e `count` é 1
                Self {
                    count: 1,
                    dados: vec![data],
                    ..Default::default()
                }
            }
            None => {
                // Se for None, `dados` é vazio e `count` é 0
                Self {
                    count: 0,
                    dados: vec![],
                    ..Default::default()
                }
            }
        }
    }
}

impl<T: Default> From<Result<T>> for Pacote<T> {
    fn from(res: Result<T, Error>) -> Self {
        match res {
            // Caso de sucesso: empacota os dados
            Ok(value) => Pacote {
                dados: vec![value],
                count: 1,
                status: 200,  // Código HTTP de sucesso
                ..Default::default()
            },
            // Caso de erro: captura a mensagem de erro
            Err(err) => Pacote {
                erros: vec![format!("{}", err)], // Formata o erro como string
                status: 500,  // Código HTTP genérico de erro no servidor
                mensagem: Some("Falha ao processar a requisição".to_string()),
                ..Default::default()
            },
        }
    }
}
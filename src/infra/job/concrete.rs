use log::info;
use serde::{Deserialize, Serialize};

/// Enquanto o trait Jober é uma interface abstrata para executar um Job (serviço),
/// o `mod concrete;` representa um módulo concreto, isto é, onde é feita a implementação
/// de cada Job, a sua rotina run é exatamente o processo que deve ser executado
/// 
/// Exemplos de jobs são "enviar um e-mail; processar um pedido; calcular o custo"

use crate::infra::job::model::Jober;
use crate::infra::email::send_email;
use super::model::Job;

#[derive(Default, Serialize, Deserialize)]
pub struct SendEmail {
    pub recipient: String,
    pub subject: String,
    pub body: String,
}

impl SendEmail {
    async fn send(self,) {
        send_email(self.recipient.as_str(), self.subject.as_str(), self.body.as_str()).await;
    }
}

#[derive(Deserialize)]
struct UserAdapter {
    _nome: String,
    _instituicao: String,
    email: String,
    _telefone: String,
}

// Este adapter deve representar exatamente o JSON que é armazenado no Content dos job
#[derive(Deserialize)]
struct JobEmailAdapter {
    usuario: UserAdapter,
    assunto: String,
    texto: String,
}

impl Jober for SendEmail {
    async fn run(job: Job) -> () {
        // é muito simples, 
        // o job contém um json com todas as variáveis por exemplo hora de execução, assunto, corpo do e-mail etc
        // ele pega o job.content e joga dentro do SendEmail, que é uma struct com as informacoes do e-mail. 
        // Esta classe intermediária possui um método run que aciona a crate de enviar email passando o contexto
        let mut runner = Self::default();
        let email_json: JobEmailAdapter = serde_json::from_value(job.context).unwrap();
        runner.recipient = email_json.usuario.email;
        runner.subject = email_json.assunto;
        runner.body = email_json.texto;


        info!(
            "🔨 Enviando e-mail para: {}\nAssunto: {}\nCorpo:...",
            runner.recipient, runner.subject
        );
        
        runner.send().await;

        info!("E-mail enviado com sucesso");
        
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct PrintReport {
    pub report_name: String,
    pub content: String,
}

impl Jober for PrintReport {
    async fn run(_job: Job)  -> () { 
        let runner = Self::default();
        // Simulação de impressão de relatório
        println!("Imprimindo relatório: {}\nConteúdo: {}", runner.report_name, runner.content);
        // Aqui você poderia adicionar a lógica real de impressão
       
    }
}
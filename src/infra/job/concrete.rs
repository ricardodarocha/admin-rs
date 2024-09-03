use crate::infra::job::model::Jober;

use super::model::Job;

pub struct SendEmail {
    pub recipient: String,
    pub subject: String,
    pub body: String,
}

impl Jober for SendEmail {
    fn run(&self, _job: Job) -> () {
        // Simulação de envio de e-mail
        println!(
            "Enviando e-mail para: {}\nAssunto: {}\nCorpo: {}",
            self.recipient, self.subject, self.body
        );
        // Aqui você poderia adicionar a lógica real de envio de e-mail
        
    }
}

pub struct PrintReport {
    pub report_name: String,
    pub content: String,
}

impl Jober for PrintReport {
    fn run(&self, _job: Job)  -> () {
        // Simulação de impressão de relatório
        println!("Imprimindo relatório: {}\nConteúdo: {}", self.report_name, self.content);
        // Aqui você poderia adicionar a lógica real de impressão
       
    }
}
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
// use log::info;

pub async fn send_email(to_email: &str, subject: &str, content: &str,) {

    let smtp_key = std::env::var("BREVOSMTP").unwrap().parse::<String>().unwrap();
    let from_email = std::env::var("BREVOREMETENTE").unwrap().parse::<String>().unwrap();
    let replay = "rickrochaso@gmail.com";
    let brevo_credencial = std::env::var("BREVOCREDENCIAL").unwrap().parse::<String>().unwrap();
    let brevo_host = std::env::var("BREVOHOST").unwrap().parse::<String>().unwrap();
    
    
    let email = Message::builder()
    .from(from_email.parse().unwrap())
    .reply_to(replay.parse().unwrap())
    .to(to_email.parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_HTML)
    .body(String::from(content))
    .unwrap();
    // info!("21");

    let creds = Credentials::new(brevo_credencial, smtp_key.to_owned());
    // info!("24");

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(brevo_host.as_str())
        .unwrap()
        .credentials(creds)
        .build();
    // info!("31");

    // Send the email
    match mailer.send(&email) { 
        Ok(message) => println!("E-mail enviado {} \n{:?}", to_email.to_owned(), message),
        Err(e) => println!("Erro ao enviar e-mail: {e:?}"),
    }   

    }
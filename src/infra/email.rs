use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(to_email: &str, subject: &str, content: &str,) {

    let smtp_key = std::env::var("SMPT").unwrap().parse::<String>().unwrap();
    let from_email = "ricardodarocha@outlook.com";
    let host = "smtp-mail.outlook.com";
    
    let email = Message::builder()
    .from(from_email.parse().unwrap())
    .reply_to(from_email.parse().unwrap())
    .to(to_email.parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_PLAIN)
    .body(String::from(content))
    .unwrap();

    let creds = Credentials::new(from_email.to_owned(), smtp_key.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("E-mail enviado {}", to_email.to_owned()),
        Err(e) => panic!("Erro ao enviar e-mail: {e:?}"),
    }   

    }
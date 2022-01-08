#![allow(warnings)]
extern crate lettre;
extern crate lettre_email;
extern crate tokio;



use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use liberrors::myErrors;


pub fn send_email(to_mail: &str, from_mail: &str,
                  body_text: &str,
                  smtp_address: &str, username: &str,
                  password:&str)  {

    let email = EmailBuilder::new()
        .to(to_mail)
        .from(from_mail)
        .subject("New coins")
        .body(body_text)
        .build()
        .unwrap()
        .into();
    let credentials = (username, password).into_credentials();
    let mut client = SmtpClient::new_simple(smtp_address)
        .unwrap()
        .credentials(credentials)
        .transport();

    let _result = client.send(email);
    println!("{:?}", _result);
}


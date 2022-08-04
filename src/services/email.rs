use crate::prelude::*;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct EmailService {}

impl EmailService {
    pub async fn send_confirmation_email(
        settings: &Arc<WispSettings>,
        email: &str,
        code: &str,
    ) -> Result<()> {
        if !settings.email_enabled {
            return Ok(());
        }

        let email = Message::builder()
            .from(settings.email_from.parse().unwrap())
            .to(email.to_string().parse().unwrap())
            .subject("wisp.pw email confirmation")
            .body(String::from("Confirmation code is ") + code)
            .unwrap();

        let creds = Credentials::new(settings.email_user.clone(), settings.email_pass.clone());
        let mailer = SmtpTransport::starttls_relay(&settings.email_host)?
            .credentials(creds)
            .build();

        mailer.send(&email)?;

        Ok(())
    }
}

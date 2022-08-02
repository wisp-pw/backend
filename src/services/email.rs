use async_smtp::{
    smtp::authentication::Credentials, Envelope, SendableEmail, SmtpClient, Transport,
};

use crate::prelude::*;

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

        let body = SendableEmail::new(
            Envelope::new(
                Some(settings.email_from.parse().unwrap()),
                vec![email.parse().unwrap()],
            )?,
            "Wisp confirmation email",
            "Code is ".to_string() + code,
        );

        let credentials =
            Credentials::new(settings.email_user.clone(), settings.email_pass.clone());

        let mut transport =
            SmtpClient::new_host_port(settings.email_host.clone(), settings.email_port)
                .credentials(credentials)
                .into_transport();

        transport.send(body).await?;
        Ok(())
    }
}

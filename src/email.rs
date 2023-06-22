use std::sync::Arc;

use base64::Engine;
use lettre::{
    message::header::{ContentType, MIME_VERSION_1_0},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

pub(crate) struct Email {}
impl Email {
    pub(crate) async fn send_registration(
        config: Arc<crate::base::Config>,
        to_email: String,
        to_username: String,
        to_hash: String,
    ) -> crate::base::Result<()> {
        let subject = format!("LEX Registration for {to_username}");
        let key = base64::engine::general_purpose::STANDARD_NO_PAD
            .encode(format!("{to_username}:{to_hash}"));
        let link = format!(
            "{}api/{}/user/activate?activation_key={}",
            config.index_link, config.api_version, key
        );

        let message = format!("
            <html>
                <head>
                    <title>File Exchange Registration for {to_username}</title>
                </head>
                <body>
                    <h3>Welcome to the File Exchange!</h3>
                    <p>To make sure that the data you entered is correct, please click the link below to activate your account</p>
                    <p>Activation: <a href='{link}'>Click here</a></p>
                </body>
            </html>
        ");

        Ok(mail(config, to_email, subject, message).await?)
    }
}

async fn mail(
    config: Arc<crate::base::Config>,
    to_email: String,
    subject: String,
    message: String,
) -> crate::base::Result<()> {
    let from = format!("SC4D File Exchange Administration <{}>", config.email_orig);
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .header(MIME_VERSION_1_0)
        .body(message)?;

    let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::unencrypted_localhost();

    // Send the email
    let send = mailer.send(email).await?;

    Ok(())
}

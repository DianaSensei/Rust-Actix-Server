use crate::SETTINGS;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::SmtpTransport;
use lettre::{Message, Transport};
use once_cell::sync::OnceCell;

static SMTP_CONNECTION: OnceCell<SmtpTransport> = OnceCell::new();
static SMTP_CONNECTION_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

pub async fn get_smtp_connection() -> Option<&'static SmtpTransport> {
    if SMTP_CONNECTION.get().is_some() {
        return SMTP_CONNECTION.get();
    }

    let initializing_mutex =
        SMTP_CONNECTION_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;

    let smtp_host = &SETTINGS.mail.host;
    let smtp_username = SETTINGS.mail.username.clone();
    let smtp_password = SETTINGS.mail.password.clone();

    if !*initialized {
        if let Ok(conn_builder) = SmtpTransport::starttls_relay(smtp_host) {
            let conn = conn_builder
                .credentials(Credentials::new(smtp_username, smtp_password))
                .build();
            if SMTP_CONNECTION.set(conn).is_ok() {
                info!("SMTP CLIENT INITIATE: [SUCCESS]");
                *initialized = true;
            }
        }
    }

    drop(initialized);
    SMTP_CONNECTION.get()
}

pub async fn send_email(email: Message) -> Response {
    let mailer = get_smtp_connection().await.unwrap();
    // Send the email
    mailer.send(&email).unwrap()
}

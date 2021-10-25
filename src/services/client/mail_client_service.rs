use lettre::transport::smtp::SmtpTransport;
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, Transport};
use once_cell::sync::OnceCell;
use crate::config;

static SMTP_CONNECTION: OnceCell<SmtpTransport> = OnceCell::new();
static SMTP_CONNECTION_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

pub async fn get_smtp_connection() -> Option<&'static SmtpTransport> {
    if let Some(_) = SMTP_CONNECTION.get() {
        return SMTP_CONNECTION.get();
    }

    let initializing_mutex = SMTP_CONNECTION_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;

    let smtp_host = &*config::CONFIG.smtp_host;
    let smtp_username = config::CONFIG.smtp_username.clone();
    let smtp_password = (config::CONFIG).smtp_password.clone();

    if !*initialized {
        if let Ok(conn_builder) = SmtpTransport::starttls_relay(smtp_host) {
            let conn = conn_builder.credentials(Credentials::new(smtp_username, smtp_password)).build();
            if let Ok(_) = SMTP_CONNECTION.set(conn) {
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
    let result = mailer.send(&email).unwrap();
    result
}

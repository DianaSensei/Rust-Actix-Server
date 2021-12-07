use crate::config;
use nats::connect;
use nats::Connection;
use once_cell::sync::OnceCell;

static NATS_CONNECTION: OnceCell<Connection> = OnceCell::new();
static NATS_CONNECTION_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

pub async fn get_nats_connection() -> Option<&'static Connection> {
    if NATS_CONNECTION.get().is_some() {
        return NATS_CONNECTION.get();
    }

    let initializing_mutex =
        NATS_CONNECTION_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;
    if !*initialized {
        if let Ok(conn) = connect(config::CONFIG.nats_url.as_str()) {
            if NATS_CONNECTION.set(conn).is_ok() {
                info!("NATS CLIENT INITIATE: [SUCCESS]");
                *initialized = true;
            }
        }
    }

    drop(initialized);
    NATS_CONNECTION.get()
}

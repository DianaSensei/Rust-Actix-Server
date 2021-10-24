use once_cell::sync::OnceCell;
use nats::connect;
use nats::Connection;
use crate::config;

static NATS_CONNECTION: OnceCell<Connection> = OnceCell::new();
static NATS_CONNECTION_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

pub async fn get_nats_connection() -> Option<&'static Connection> {
    if let Some(_) = NATS_CONNECTION.get() {
        return NATS_CONNECTION.get();
    }

    let initializing_mutex = NATS_CONNECTION_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;
    if !*initialized {
        if let Ok(conn) = connect(config::CONFIG.nats_url.as_str()) {
            if let Ok(_) = NATS_CONNECTION.set(conn) {
                *initialized = true;
            }
        }
    }

    drop(initialized);
    NATS_CONNECTION.get()
}
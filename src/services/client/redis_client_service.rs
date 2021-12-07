use crate::config;
use once_cell::sync::OnceCell;
use redis::aio::ConnectionManager;

static REDIS_CLIENT: OnceCell<ConnectionManager> = OnceCell::new();
static REDIS_CLIENT_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

pub async fn get_redis_connection() -> Option<&'static ConnectionManager> {
    if REDIS_CLIENT.get().is_some() {
        return REDIS_CLIENT.get();
    }

    let initializing_mutex =
        REDIS_CLIENT_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;
    if !*initialized {
        if let Ok(client) = redis::Client::open(config::CONFIG.redis_url.as_str()) {
            if let Ok(conn) = client.get_tokio_connection_manager().await {
                if REDIS_CLIENT.set(conn).is_ok() {
                    info!("REDIS CLIENT INITIATE: [SUCCESS]");
                    *initialized = true;
                }
            }
        }
    }

    drop(initialized);
    REDIS_CLIENT.get()
}

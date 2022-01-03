use crate::utils::hasher::default_hasher_scheme_version;
use crate::utils::project_profile::get_profile;
use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use std::string::ToString;

const CONFIG_FILE_PATH: &str = "./config/default.toml";
const CONFIG_FOLDER_PATH: &str = "./config/";

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub datasource: DataSource,
    pub message_broker: MessageBroker,
    pub rules: Vec<Rule>,
    pub log: Log,
    pub tracer: Tracer,
    pub cargo_pkg_name: String,
    pub hasher: Hasher,
    pub mail: Mail,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Hasher {
    pub scheme_version: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mail {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tracer {
    pub jaeger: Jaeger,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jaeger {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub listen_port: u16,
    pub listen_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DataSource {
    pub database: Database,
    pub redis: Redis,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Redis {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageBroker {
    pub nats: Nats,
    pub kafka: Kafka,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Nats {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Kafka {
    pub url: String,
    pub message_timeout: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    pub rule_set: Vec<String>,
}

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::new().expect("config can be loaded"));

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = get_profile().to_string().to_lowercase();
        let mut s = Config::new();
        s.set_default(
            "hasher.scheme_version",
            default_hasher_scheme_version().to_string(),
        )?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FOLDER_PATH, env)))?;

        // This makes it so "EA_SERVER__PORT overrides server.port
        s.merge(Environment::default())?;
        // s.merge(Environment::with_prefix("APP").separator("__"))?;

        s.try_into()
    }
}

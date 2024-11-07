use serde_derive::{Deserialize};

#[derive(Clone, Deserialize)]
pub struct Config {
    pub env: String,
    pub host: String,
    pub port: u16,
    pub secret: String,
    pub pg: deadpool_postgres::Config,
    pub redis: Redis,
}

impl Config {
    pub fn from_env() -> Self {
        // // @TODO добавить какой то флаг, чтобы только локально вычитывалось
        // from_filename(format!(".env.{}", std::env::var("ENV").unwrap())).ok();

        let cfg = config::Config::builder()
            .add_source(
                config::Environment::with_prefix("SWD")
                    .try_parsing(true)
                    .separator("_")
            )
            .build()
            .unwrap();

        cfg.try_deserialize::<Config>().unwrap()
    }
}

#[derive(Clone, Deserialize)]
pub struct Redis {
    pub port: u16,
    pub password: String,
}
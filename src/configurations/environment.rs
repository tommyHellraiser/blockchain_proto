use std::io::Read;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use serde::Deserialize;

lazy_static!(
    static ref ENVIRONMENT: Environment = Environment::new();
);

pub struct Environment {
    inner: RwLock<EnvironmentInner>
}

#[derive(Deserialize, Debug)]
struct EnvironmentInner {
    db_addr: String,
    api_base_addr: String,
    api_port: u16,
    api_workers: usize,
    // difficulty: u128
    reset_schema: bool
}

impl Environment {

    fn new() -> Self {
        Self {
            inner: RwLock::new(
                load_env_config()
            )
        }
    }

    pub fn instance() -> &'static Self {
        &ENVIRONMENT
    }

    pub async fn get_db_addr(&self) -> String {
        self.inner.read().await.db_addr.clone()
    }

    pub async fn get_api_config(&self) -> (String, u16, usize) {
        let api_base_addr = self.inner.read().await.api_base_addr.clone();
        let api_port = self.inner.read().await.api_port;
        let api_workers = self.inner.read().await.api_workers;

        (api_base_addr, api_port, api_workers)
    }

    pub async fn reset_schema(&self) -> bool {
        self.inner.read().await.reset_schema
    }
}

fn load_env_config() -> EnvironmentInner {

    let mut reader = std::fs::OpenOptions::new()
        .read(true)
        .open("config/env.json").unwrap();
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();

    serde_json::from_str::<EnvironmentInner>(&buffer).unwrap()

}

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
    pub db_addr: String,
    pub difficulty: u128
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

    pub async fn print_content(&self) {
        let asd = self.inner.read().await;
        dbg!(asd);
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

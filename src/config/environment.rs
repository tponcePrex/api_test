use std::fs::File;
use std::io::ErrorKind::InvalidData;
use tokio::sync::RwLock;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref ENVIRONMENT_CONFIG: EnvironmentConfig = EnvironmentConfig::new();
}

pub struct EnvironmentConfig {
    inner: RwLock<EnvironmentConfigInner>,
}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentConfigInner {
    api_url: String,
    api_port: String,
    online_api_url: String,
    online_api_token: String,
}

impl EnvironmentConfig {

    pub fn new() -> Self {
        Self {
            //  Will panic if can't open file
            inner: RwLock::new(Self::load().unwrap())
        }
    }

    fn load() -> std::io::Result<EnvironmentConfigInner> {

        let file = File::open("config/config.json")
            //.map_err(|e| new_error!(e, ErrorTypes::OpenFile))?;
            //  TODO: find a way to map the error into a custom error
            .map_err(|e| std::io::Error::new(InvalidData, format!("{e}")))?;

        //  TODO: same here, map the error into a custom error
        match serde_json::from_reader::<_, EnvironmentConfigInner>(file) {
            Ok(config) => Ok(config),
            Err(e) => Err(std::io::Error::new(InvalidData, format!("{e}")))
        }
    }

    pub fn instance() -> & 'static Self { &ENVIRONMENT_CONFIG }

    pub async fn get_api_bind(&self) -> String {
        let api_url = self.inner.read().await.api_url.clone();
        let api_port = self.inner.read().await.api_port.clone();

        let api_bind = format!("{api_url}:{api_port}");

        api_bind
    }

    pub async fn get_online_api_url(&self) -> String {
        self.inner.read().await.online_api_url.clone()
    }

    pub async fn get_online_api_token(&self) -> String {
        self.inner.read().await.online_api_token.clone()
    }
}

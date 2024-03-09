use once_cell::sync::Lazy;

use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub cookie_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub spotify: SpotifyConfig,
    pub app: AppConfig,
}

pub static CONFIGURATION_INSTANCE: Lazy<Configuration> =
    Lazy::new(|| load_config_file().expect("Could not load config.toml"));

impl Configuration {
    pub fn read() -> &'static Configuration {
        &CONFIGURATION_INSTANCE
    }

    pub fn app() -> &'static AppConfig {
        &CONFIGURATION_INSTANCE.app
    }

    pub fn spotify() -> &'static SpotifyConfig {
        &CONFIGURATION_INSTANCE.spotify
    }
}

fn load_config_file() -> Result<Configuration, Error> {
    let config_file = std::fs::read_to_string("config.toml")?;
    let config = toml::from_str(&config_file)?;
    Ok(config)
}

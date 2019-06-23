use crate::Error;
use serde::Deserialize;
use config::{Config as RawConfig, Environment};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub discord_api_token: String,
    pub db_location: String,
}
impl Config {
    pub fn from_env() -> Result<Self, Error> {
        let env = Environment::new();

        let mut raw_config = RawConfig::new();
        raw_config.merge(env)?;

        let config = raw_config.try_into::<Config>()?;
        Ok(config)
    }
}

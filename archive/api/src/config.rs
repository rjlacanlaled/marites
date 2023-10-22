use crate::{ Error, Result };

use std::{ env, sync::OnceLock };

pub fn config() -> &'static Config {
    static INSTACE: OnceLock<Config> = OnceLock::new();

    INSTACE.get_or_init(||
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - Failed to load config: {ex:?}")
        })
    )
}

#[allow(non_snake_case)]
pub struct Config {
    // -- Db
    pub DB_URL: String,

    // -- Web
    pub WEB_FOLDER: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            // -- Db
            DB_URL: get_env("SERVICE_DB_URL")?,

            // -- Web
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

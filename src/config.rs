use crate::{miscs::get_env, Result};

use core::panic;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|e| panic!("Failed to load config: {e:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub JWT_SECRET: String,
}

impl Config {
    fn load_from_env() -> Result<Self> {
        Ok(Self {
            JWT_SECRET: get_env("JWT_SECRET")?,
        })
    }
}

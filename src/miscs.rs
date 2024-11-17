use crate::{Error, Result};

pub fn get_env(name: &str) -> Result<String> {
    std::env::var(name).map_err(|_| Error::MissingEnv(name.to_string()))
}

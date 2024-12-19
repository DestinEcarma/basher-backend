use crate::miscs::get_env;
use crate::Result;

#[allow(non_snake_case)]
pub struct Config {
    pub URL: String,
    pub USER: String,
    pub PASS: String,
    pub NS: String,
    pub DB: String,
}

impl Config {
    pub fn load_from_env() -> Result<Self> {
        Ok(Self {
            URL: get_env("SURREAL_URL")?,
            USER: get_env("SURREAL_USER")?,
            PASS: get_env("SURREAL_PASS")?,
            NS: get_env("SURREAL_NS")?,
            DB: get_env("SURREAL_DB")?,
        })
    }
}

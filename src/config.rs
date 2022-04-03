use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) db_user: String,
    pub(crate) db_host: String,
    pub(crate) db_password: String,
    pub(crate) db_name: String,
    pub(crate) db_port: u16,
    pub(crate) ts_name: String,
    pub(crate) ts_password: String,
    pub(crate) ts_port: u16
}

impl Config {
    pub(crate) fn new() -> Self {
        Config::from_config_file("src/config.toml").unwrap()
    }
}

use crate::cli::Cli;
use anyhow::{anyhow, Result};
use figment::{
    providers::{Env, Format, Serialized, Yaml},
    Figment,
};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub vault_cacert: Option<PathBuf>,
    pub vault_client_cert: Option<PathBuf>,
    pub vault_client_key: Option<PathBuf>,
    pub vault_path: String,
    pub vault_addr: String,
    pub vault_token_path: PathBuf,
    pub vault_pubkeys_json_path: PathBuf,
    #[serde(default = "default_vault_max_concurrent_requests")]
    pub vault_max_concurrent_requests: usize,
}

fn default_vault_max_concurrent_requests() -> usize {
    20
}

impl Config {
    pub fn new(args: &Cli) -> Result<Self> {
        let mut config = Figment::new();
        if let Some(config_file) = &args.config {
            config = config.merge(Yaml::file(config_file));
        }

        let config = config
            .merge(Env::prefixed("VAULT_"))
            .merge(Serialized::from(&args, ""));

        let has_vault_cacert: bool = config.extract_inner::<PathBuf>("vault_cacert").is_ok();
        let has_vault_client_cert = config.extract_inner::<PathBuf>("vault_client_cert").is_ok();
        let has_vault_client_key = config.extract_inner::<PathBuf>("vault_client_key").is_ok();

        if has_vault_cacert != has_vault_client_cert || has_vault_cacert != has_vault_client_key {
            return Err(anyhow!("vault_cacert, vault_client_cert, and vault_client_key must be set together or not at all"));
        }
        Ok(config.extract()?)
    }
}

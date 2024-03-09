use anyhow::{Context, Ok, Result};
use config::ConfigError;
use once_cell::sync::Lazy;
use serde::Deserialize;

use aws_sdk_sqs::types::Message;
use dotenvy::dotenv;

pub fn extract_job_id_from_sqs_message(message: &Message) -> Result<String> {
    Ok(message.body().context("")?.to_string())
}

#[derive(Deserialize)]
pub struct Config {
    pub sqs_endpoint: String,
    pub container_image_name: String,
    pub github_endpoint: String,
    pub github_user_name: String,
    pub github_pat: String,
    pub runner_name_prefix: String,
    pub runner_group_name: String,
    pub runner_work_dir: String,
}

impl Config {
    pub fn get_env() -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        cfg.try_deserialize()
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().expect("Cannot find .env file");
    Config::get_env().expect("Failed to load config")
});

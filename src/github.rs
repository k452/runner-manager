use anyhow::{Ok, Result};
use reqwest::{cookie::Jar, Client};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::utils::CONFIG;

#[derive(Deserialize)]
struct GithubApiResp {
    encoded_jit_config: String,
}

pub async fn generate_jit_config(job_id: &str) -> Result<String> {
    let cookie_store = Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::clone(&cookie_store))
        .build()?;

    let req_body_json = json!({
        "name": format!("{0}-{job_id}", CONFIG.runner_name_prefix),
        "runner_group_name": format!("{0}", CONFIG.runner_group_name),
        "runner_group_id": 1,
        "labels": vec![format!("[\"{0}\"]", CONFIG.runner_name_prefix)],
        "work_folder": format!("{0}", CONFIG.runner_work_dir),
    });

    let res = client
        .post(format!("{0}", CONFIG.github_endpoint))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "runner-manager")
        .header("Authorization", format!("Bearer {0}", CONFIG.github_pat))
        .json(&req_body_json)
        .send()
        .await?;

    Ok(res.json::<GithubApiResp>().await?.encoded_jit_config)
}

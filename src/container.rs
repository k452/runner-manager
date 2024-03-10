use std::process::Command;

use crate::utils::CONFIG;

pub fn run(job_id: &str, jit_config: &str) {
    let _output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "docker run --name gha-runner-{job_id} --rm --detach -e JIT_CONFIG={jit_config} {0}",
            CONFIG.container_image_name
        ))
        .output()
        .expect("failed to run container.\n id=[&job_id]");
}

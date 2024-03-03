use std::{env, process::Command};

pub fn run(job_id: String) -> Vec<u8> {
    let container_image_name = env::var("CONTAINER_IMAGE_NAME").expect("no such var.");

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "docker run --name gha-runner-{job_id} --rm --detach {container_image_name}"
        ))
        .output()
        .expect("failed to run container.\n id=[&job_id]");

    output.stdout
}

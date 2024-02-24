use std::process::Command;

pub fn run() -> Vec<u8> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("docker ps")
        .output()
        .expect("failed to execute process");

    output.stdout
}

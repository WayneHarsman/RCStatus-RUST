use std::process::{Command};
use utils::error::Result;

pub fn validate_remote(remote: &str) -> Result<bool> {
    let remote_base = get_remote_base(remote);

    let status = Command::new("rclone")
        .arg("lsf")
        .arg(remote_base)
        .stdout(std::process::Stdio::null())
        .status()
        .expect("Failed to execute rclone");

    if !status.success() {
        return Ok(false)
    }

    Ok(true)
}

pub fn validate_remote_path(remote_path: &str) -> Result<bool> {
    let status = Command::new("rclone")
        .arg("lsf")
        .arg(remote_path)
        .status()
        .expect("Failed to execute rclone");

    if !status.success() {
        return Ok(false)
    }

    Ok(true)
}

fn get_remote_base(remote: &str) -> String {
    remote.split(':').collect::<Vec<&str>>()[0].to_string() + ":"
}
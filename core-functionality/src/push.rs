use crate::common::validate_remote;
use utils::error::{Result, Error};
use std::process::{Command};

pub fn push(_target: String) -> Result<()> {
    let config = match database::functions::retrieve_config(&_target){
        Some(config) => config,
        None => {
            println!("Target is not being tracked");
            return Err(Error::new("Target is not being tracked"));
        }
    };

    // Check if remote is still valid
    if !validate_remote(&config.remote)? {
        println!("Specified remote is no longer valid");
        return Err(Error::new("Specified remote is no longer valid"));
    }

    let status = Command::new("rclone")
        .arg("sync")
        .arg(&config.target)
        .arg(&config.remote)
        .status()
        .expect("Failed to execute rclone");

    if !status.success() {
        return Err(Error::new("Failed to push"));
    }


    Ok(())
}
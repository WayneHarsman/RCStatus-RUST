use utils::error::{Error, Result};
use std::process::{Command};
use crate::common::validate_remote;

pub fn fetch(_target: String) -> Result<()>{
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
        return Ok(())
    }

    let status = Command::new("rclone")
        .arg("sync")
        .arg(&config.remote)
        .arg(&config.target)
        .status()
        .expect("Failed to execute rclone");

    if !status.success() {
        println!("Failed to fetch");
        return Ok(())
    }


    Ok(())
}
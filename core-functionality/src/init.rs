use std::path::PathBuf;
use utils::error::{Result, Error};
use super::common::validate_remote;


pub fn init(target: String, remote: String) -> Result<()> {
    let path = PathBuf::from(&target);

    if !path.exists(){
        return Err(Error::new("Target directory does not exist"));
    }

    if !validate_remote(&remote)? {
        return Err(Error::new("Specified remote is not valid"));
    }

    if database::functions::contains_config(&target)? {
        return Err(Error::new("This target is already being tracked"));
    }

    let new_config = database::models::NewConfig {
        target: target,
        remote: remote
    };


    database::functions::add_config(&new_config);

    println!("Config added");
    Ok(())
}
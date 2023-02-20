use utils::error::{Error, Result};


pub fn forget(target: String) -> Result<()> {
    let config = match  database::functions::retrieve_config(&target){
        Some(config) => config,
        None => {
            println!("Target is not being tracked");
            return Err(Error::new("Target is not being tracked"));
        }
    };

    if !utils::cmd::ask_for_permission(&format!("Are you sure you want to disable sync for [{}]?", config.target))?{
        println!("Aborting");
        return Ok(())
    }

    database::functions::delete_config(&config);

    println!("Config deleted");

    Ok(())
}
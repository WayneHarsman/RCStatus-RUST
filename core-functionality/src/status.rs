use utils::error::Result;
use database::functions as database;

pub fn status(_target: String) -> Result<()> {
    let config = match database::retrieve_config(&_target){
        Some(config) => config,
        None => {
            println!("Target is not being tracked");
            return Ok(())
        }
    };

    println!("[{}] --> [{}]", config.remote, config.target);

    Ok(())
}
use super::error::Result;
use text_io::read;

pub fn ask_for_permission(request: &str) -> Result<bool>{
    println!("{}: (y/n)", request);
    let choice: char = read!();

    match choice {
        'y' => Ok(true),
        'n' => Ok(false),
        _ => {
            println!("Invalid choice");
            Ok(false)
        }
    }
}
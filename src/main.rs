
use utils::error::Result;

fn main() -> Result<()> {
    cli::cli_match()?;

    Ok(())
}
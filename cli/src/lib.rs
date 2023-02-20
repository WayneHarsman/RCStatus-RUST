mod arg_helpers;

use clap::{Args, Parser, Subcommand};
use arg_helpers::get_path_as_string;

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new repository
    Init (InitDirectory),

    /// Get synchronization status of selected directory
    Status (Status),

    /// Synchronize selected directory
    Fetch (Fetch),

    /// Push changes to remote
    Push (Push),
    
    /// Disable synchronization of selected directory
    Forget (Forget)
}

#[derive(Args, Debug)]
pub struct InitDirectory {
    /// Target remote to fetch from
    #[arg(required = true)]
    pub remote: String,

    /// The directory to initialize the repository in
    #[arg(short, default_value_t = get_path_as_string(), required = false)]
    pub target: String,

}

#[derive(Args, Debug)]
pub struct Status {
    /// Target directory to check status of
    #[arg(short, default_value_t = get_path_as_string(), required = false)]
    pub target: String
}

#[derive(Args, Debug)]
pub struct Fetch {
    /// Target directory to sync
    #[arg(short, default_value_t = get_path_as_string(), required = false)]
    pub target: String
}

#[derive(Args, Debug)]
pub struct Push {
    /// Target directory to sync
    #[arg(short, default_value_t = get_path_as_string(), required = false)]
    pub target: String
}

#[derive(Args, Debug)]
pub struct Forget {
    /// Target directory to sync
    #[arg(short, default_value_t = get_path_as_string(), required = false)]
    pub target: String
}


pub fn cli_match() -> utils::error::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Init(x) => core::init::init(x.target, x.remote),
        Commands::Status(x) => core::status::status(x.target),
        Commands::Fetch(x) => core::fetch::fetch(x.target),
        Commands::Push(x) => core::push::push(x.target),
        Commands::Forget(x) => core::forget::forget(x.target),
    }
}

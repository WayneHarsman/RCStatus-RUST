use clap::{Args, Parser, Subcommand};
use crate::arg_helpers::get_path_as_string;

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
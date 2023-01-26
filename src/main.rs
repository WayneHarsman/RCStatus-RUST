mod cmd;
mod arg_helpers;
mod models;

use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf};
use std::process::{Command, Stdio};
use clap::Parser;
use cmd::Cli;
use crate::cmd::Commands;

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Init(x) => init(x.target, x.remote),
        Commands::Status(x) => status(x.target),
        Commands::Fetch(x) => sync(x.target),
        Commands::Push(x) => push(x.target),
    }
}

fn init(target: String, remote: String,) {
    let mut path = PathBuf::new();
    path.push(&target);
    path.push(".rcstat");

    if path.is_dir() {
        println!("Directory is already tracked");
        return;
    }

    // Check if remote is valid
    print!("Checking remote [{}]:\t", remote);
    let status = Command::new("rclone")
        .arg("lsf")
        .arg(&remote)
        .stdout(Stdio::null())
        .status().unwrap();
    if !status.success() {
        println!("Invalid remote");
        return;
    }
    println!("Done");

    std::fs::create_dir(&path).unwrap();

    path.push("config.txt");
    let file = File::create(path).unwrap();
    let conf = models::Config {
        remote,
        target
    };
    serde_json::to_writer(file, &conf).unwrap();
    println!("Done");
}

fn status(target: String) {
    let mut path = PathBuf::new();
    path.push(target);
    path.push(".rcstat");

    if !path.is_dir() {
        println!("Directory is not tracked");
        return;
    }

    path.push("config.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let conf: models::Config = serde_json::from_reader(reader).unwrap();
    println!("[remote: {}] --> [target: {}]", conf.remote, conf.target);
}

fn sync(target: String) {
    let mut path = PathBuf::new();
    path.push(target);
    path.push(".rcstat");

    if !path.is_dir() {
        println!("Directory is not tracked");
        return;
    }

    path.push("config.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let conf: models::Config = serde_json::from_reader(reader).unwrap();
    let status = Command::new("rclone")
        .arg("sync")
        .arg(&conf.remote)
        .arg(&conf.target)
        .status().unwrap();
    if !status.success() {
        eprintln!("Failed to sync {} with {}", conf.target, conf.remote)
    }
}


fn push(target: String) {
    let mut path = PathBuf::new();
    path.push(target);
    path.push(".rcstat");

    if !path.is_dir() {
        println!("Directory is not tracked");
        return;
    }

    path.push("config.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let conf: models::Config = serde_json::from_reader(reader).unwrap();
    let status = Command::new("rclone")
        .arg("sync")
        .arg(&conf.target)
        .arg(&conf.remote)
        .status().unwrap();
    if !status.success() {
        eprintln!("Failed to push changes from {} with {}", conf.target, conf.remote)
    }
}


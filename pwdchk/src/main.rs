//! Main function module

mod account; //reference to account module
mod error;

use account::*;
use clap::{Args, Parser, Subcommand, ArgGroup};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version, author, about)]
struct AppArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check duplicate passwords from command line
    Group(GroupArgs),
}

#[derive(Args)]
#[clap(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["account", "file"]),
))]
struct GroupArgs {
    #[clap(required = false)]
    /// Account to check
    account: Vec<Account>,
    #[clap(short, long)]
    /// Load passwords from a file
    file: Option<PathBuf>,
}

fn main() -> Result<(), error::Error> {
    let args = AppArgs::parse(); //get command line arguments
    match args.command {
        Command::Group(args) => {  
            match args.file {
                Some(path_) => {
                    let accounts = Account::from_file(&path_);
                    let hash = Account::group(accounts.unwrap());
                    for key in hash.keys() {
                        let logins = hash.get(key).unwrap().join(", ");
                        println!("Password {} used by {}", key, logins);
                    }
                },
                None => {
                    let hash = Account::group(args.account);
                    for key in hash.keys() {
                        let logins = hash.get(key).unwrap().join(", ");
                        println!("Password {} used by {}", key, logins);
                    }
                }
            }
        }
    }
    Ok(())
}
  
//! Main function module
mod account; //reference to account module
use account::*; //use impletentations of account module
mod error; //import error module
use error::Error; //to use directly Error structure
mod hipb; //import hipb module

use clap::{ArgGroup, Args, Parser, Subcommand};
use hipb::all_sha1_timed;
use std::{collections::HashMap, path::PathBuf};

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

fn main() -> Result<(), Error> {
    let args = AppArgs::parse(); //get command line arguments
    match args.command {
        Command::Group(args) => {
            let hash: HashMap<&str, Vec<&str>>; //variable to save hash value
            match args.file {
                Some(path_) => {
                    let accounts = Account::from_file(&path_)?;
                    //Load hash table from file
                    hash = Account::group(&accounts); //give reference of accounts to group function
                                                      //Print passwords with associated logins
                    for key in hash.keys() {
                        let logins = hash.get(key).unwrap().join(", ");
                        println!("Password {} used by {}", key, logins);
                    }
                    all_sha1_timed(&accounts); //test execution time of all_sha1 function
                } //accounts variable dies here
                None => {
                    // Load hash table from args command line
                    hash = Account::group(&args.account);
                    //Print passwords with associated logins
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

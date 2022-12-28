//! Main function module
mod account; //reference to account module
use account::*; //use impletentations of account module
mod error; //import error module
use error::Error; //to use directly Error structure
mod hipb; //import hipb module
mod scanner; //to use scanner/mod.rs and scanner/net.rs
use clap::{ArgGroup, Args, Parser, Subcommand};
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
    /// Check if some passwords of accounts have been leaked (as in Group command accounts can be direclty on
    /// command line or in file so Hipb have the same arguments Groupargs as Group command)
    Hipb(GroupArgs),
    ///Subcommand ping with PingArgs which is a structure of String (host adress) and u16 (port number)
    Ping(PingArgs),
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

///Structure for ping subcommand arguments
#[derive(Args)]
struct PingArgs {
    ///Host and port are the two arguments for ping command
    host: String,
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = AppArgs::parse(); //get command line arguments
    match args.command {
        Command::Group(args) => {
            let hash: HashMap<&str, Vec<&str>>; //variable to save hash value
            match args.file {
                Some(path_) => {
                    let accounts = Account::from_file(&path_)?;
                    //Load hash table from file
                    hash = Account::group(&accounts); //give reference of accounts to group function

                    for key in hash.keys() {
                        let logins = hash.get(key).unwrap().join(", ");
                        println!("Password {} used by {}", key, logins);
                    }
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
        //Hipb subcommand option : check if passwords of accounts in file or directly in command line have been leaked
        Command::Hipb(args) => match args.file {
            Some(path_) => {
                let accounts = Account::from_file(&path_)?; //load accounts from file if there is file argument
                let checked_accounts = hipb::check_accounts(&accounts);
                println!("{checked_accounts:#?}");
            }
            None => {
                let checked_accounts = hipb::check_accounts(&args.account); //load accounts from command line otherwise
                println!("{checked_accounts:#?}");
            }
        },
        //Ping subcommand : check if a port with a given host adress and port number on the command line is open or closed
        Command::Ping(args) => {
            let is_connexion_ok =
                scanner::net::net::tcp_ping(&args.host.as_str(), args.port).await?;
            if is_connexion_ok {
                println!("{}:{} is open", args.host, args.port);
            } else {
                println!("{}:{} is closed", args.host, args.port);
            }
        }
    }
    Ok(())
}

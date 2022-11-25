//! Main function module

mod account; //reference to account module
mod error;

use account::*;
use clap::{Args, Parser, Subcommand};

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
struct GroupArgs {
    #[clap(required = true)]
    /// Account to check
    account: Vec<Account>,
}

fn main() -> Result<(), error::Error::NoColon> {
    let args = AppArgs::parse(); //get command line arguments
    match args.command {
        Command::Group(args) => {   //args contain Account structures
           let hash = Account::group(args.account);
           for key in hash.keys() {
               let logins = hash.get(key).unwrap().join(", ");
               println!("Password {} used by {}", key, logins);
           }
           
           Ok(())
        }
    }
}
  
  
  


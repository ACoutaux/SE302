//! Main function module

mod account; //reference to account module
use account::*; //use impletentations of account module

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
fn main() -> Result<(), NoColon> {
    let args = AppArgs::parse();
    match args.command {
        Command::Group(args) => {
            // args is of type GroupArgs here
            let hash = Account::group(args.account);
            //Print passwords with associated logins
            for key in hash.keys() {
                let logins = hash.get(key).unwrap().join(", ");
                println!("Password {} used by {}", key, logins);
            }
        }
    }
    Ok(())
}

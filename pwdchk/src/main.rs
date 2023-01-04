//! Main function module
mod account; //reference to account module
use account::*; //use impletentations of account module
mod error; //import error module
use error::Error; //to use directly Error structure
mod hipb; //import hipb module
mod scanner; //to use scanner/mod.rs and scanner/net.rs
use scanner::IdentificationResult;
use crate::scanner::net::net_::expand_net;
use clap::{ArgGroup, Args, Parser, Subcommand};
use scanner::net::net_::tcp_mping;
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
    ///Subcommand ping with PingArgs which is a structure of two strings which represent host list and port list
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

#[derive(Args)]
///Structure for ping subcommand arguments
struct PingArgs {
    ///Host list and port list are the two arguments for ping command
    host: String,
    port: String,

    #[clap(short, long)] //this line allows to write -o in the command line to get the open_only option
    #[clap(required = false)] //open_only argument is not mandatory
    open_only: bool, //argument to keep only the print of open ports option
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
            let host_list: Vec<&str> = args.host.split(',').collect(); //list of str hosts from command line
            let mut res_vec: Vec<&str> = vec![];
            //Apply expand_net function on hosts to get all possible adresses with CIDR notation
            let exp_vec: Vec<Vec<String>> = host_list.iter().map(|x| expand_net(x)).collect();
            //Convert string in &str elements in exp_vec vector
            let exp_vec: Vec<Vec<&str>> = exp_vec
                .iter()
                .map(|x| x.iter().map(|y| y.as_str()).collect())
                .collect();
            //Build the &str vector with append method
            for mut vec in exp_vec {
                res_vec.append(&mut vec);
            }

            let port_list: Vec<u16> = args //list of u16 ports from command line
                .port
                .split(',')
                .map(|x| x.parse::<u16>().unwrap())
                .collect();

            //Get the connexion results
            let connexion_results = tcp_mping(&res_vec, &port_list).await;
            for res in connexion_results {
                match res.2 {
                    //match on the Result<IdentificationResult,Error>
                    Ok(res_id) => match res_id {
                        IdentificationResult::NoWelcomeLine  => println!("{}:{} is open", res.0, res.1),
                        IdentificationResult::WelcomeLine(s) => println!("{}:{} is open : {}", res.0, res.1,s),
                        IdentificationResult::ConnexionRefused =>  
                            if args.open_only { //take in account the open_only option to not print the closed ports
                            } else {
                                println!("{}:{} is closed", res.0, res.1);
                            }                    
                    }
                    //Match on the error type (Ioerror for wrong adresses and timeout errors)
                    Err(e) => match e {
                        Error::IoError(_) => {
                            if args.open_only {
                            } else {
                                println!("{}:{} is not a valid adress", res.0, res.1);
                            }
                        }
                        Error::Timeout(_) => {
                            if args.open_only {
                            } else {
                                println!("{}:{} timed out", res.0, res.1);
                            }
                        }

                        _ => {
                            if args.open_only {
                            } else {
                                println!("{}:{} -> error", res.0, res.1);
                            }
                        }
                    },
                }
            }
        }
    }
    Ok(())
}

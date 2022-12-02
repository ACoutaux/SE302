//! This module implements account (login + password) structure
use super::error; //to use content of error module in this module
use std::collections::HashMap;
use std::str::FromStr;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Clone, Debug)] //with this line debug trait can be used by Account structure
pub struct Account {
    login: String,
    password: String,
}

///Implements Account structure associated functions
impl Account {
    ///Builds a hasmap with password keys and login values from an accounts vector
    pub fn group(accounts: Vec<Account>) -> HashMap<String, Vec<String>> {
        let mut accounts_map: HashMap<String, Vec<String>> = HashMap::new();
        for account in accounts.iter() {
            let mut current_log: Vec<String> = vec![];
            current_log.push(account.login.clone());
            accounts_map
                .entry(account.password.clone())
                .and_modify(|e| e.push(account.login.clone()))
                .or_insert(current_log);
        }
        accounts_map.retain(|_, v| v.len() > 1);
        accounts_map
    }

    ///Constructor for Account structure
    pub fn new(login: &str, password: &str) -> Self {
        Account {
            login: login.to_string(),
            password: password.to_string(),
        }
    }

    ///Builds and returns an account from a string
    pub fn from_string(s: &str) -> Self {
        Account::new(s.split_once(':').unwrap().0, s.split_once(':').unwrap().1)
    }

    ///Load accounts from text file (one account per line)
    pub fn from_file(filename: &Path) -> Result<Vec<Account>, error::Error> {
        let f = File::open(Path::new(filename))?; //open file indicated by path argument and returns error otherwise
        let f = BufReader::new(f); //declare read buffer on file (mask previous f value)
        let v = f.lines().collect::<Result<Vec<_>,std::io::Error>>(); //collect lines in a result where error is of type io::Error
        let v = v.map_err(error::Error::from)?; //returns an io error from Error structure if an error is detected in v
        let accounts = v.iter().map(|x|Self::from_str(x.as_str())).collect::<Result<Vec<_>,error::Error>>()?; //returns a nocolon error from Error structure if an error is detected in v
 
        Ok(accounts)
    }
}

impl FromStr for Account {
    type Err = error::Error;
    ///Returns an Account structure if input string contains ':' and an error otherwise
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            Ok(Self::from_string(s))
        } else {
            Err(error::Error::NoColon)
        }
    }
}

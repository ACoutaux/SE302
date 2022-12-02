//! This module implements account (login + password) structure
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)] //with this line debug trait can be used by Account structure
pub struct Account {
    login: String,
    password: String,
}

#[derive(Debug)]
pub struct NoColon;

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
}

impl FromStr for Account {
    type Err = NoColon;
    ///Returns an Account structure if input string contains ':' and an error otherwise
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            Ok(Self::from_string(s))
        } else {
            Err(NoColon)
        }
    }
}

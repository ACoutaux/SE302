//! This module implements account (login + password) structure 

use super::error::Error; //import structure from error module
use std::str::FromStr;
use std::collections::HashMap;
//use std::error::Error;
//use std::fmt::Display;

#[derive(Clone)]
#[derive(Debug)] //with this line debug trait can be used by Account structure
pub struct Account{
    login: String,
    password: String,
}

/*impl Display for NoColon {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error")
  }
}

impl Error for NoColon {
}*/

impl Account {
  pub fn new(login: &str, password: &str) -> Self {
    Account {login: login.to_string(), password: password.to_string() }
  }

  pub fn from_string(s: &str) -> Self {
    Account {login: s.split_once(':').unwrap().0.to_string(), password: s.split_once(':').unwrap().1.to_string()}
  }

  pub fn group(accounts: Vec<Account>) -> HashMap<String, Vec<String>> {
    let mut accounts_map: HashMap<String, Vec<String>> = HashMap::new();
    for account in accounts.iter() {
      let mut current_log: Vec<String> = vec![];
      current_log.push(account.login.clone());
      accounts_map.entry(account.password.clone()).and_modify(|e|{e.push(account.login.clone())}).or_insert(current_log);
    }
    accounts_map.retain(|_,v| {v.len()>1});
    accounts_map
  }
}

impl FromStr for Account {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            Ok(Self::from_string(s))
        } else {
            Err(Error::NoColon)
        }
    }
}

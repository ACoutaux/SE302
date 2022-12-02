//! This module implements account (login + password) structure 
use super::error; //to use content of error module in this module
use std::str::FromStr;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Clone)]
#[derive(Debug)] //with this line debug trait can be used by Account structure
pub struct Account{
    login: String,
    password: String,
}

impl Account {
  pub fn new(login: &str, password: &str) -> Self {
    Account {login: login.to_string(), password: password.to_string() }
  }

  pub fn from_string(s: &str) -> Self {
    Account::new(s.split_once(':').unwrap().0, s.split_once(':').unwrap().1)
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

  ///Load accounts from text file (one account per line)
  pub fn from_file(filename: &Path) -> Result<Vec<Account>, error::Error> {
    let f = File::open(Path::new(filename))?; //open file indicated by path argument and returns error otherwise
    let f = BufReader::new(f); //declare read buffer on file (mask previous f value)
    let mut accounts: Vec<Account> = vec![]; //init accounts vector
    for line in f.lines() {
      accounts.push(Self::from_str(&line.unwrap()).unwrap())
    }
    Ok(accounts) 
  }
}

impl FromStr for Account {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            Ok(Self::from_string(s))
        } else {
            Err(error::Error::NoColon)
        }
    }
}

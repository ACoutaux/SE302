//! This module implements account (login + password) structure

#[derive(Clone, Debug)] //with this line debug trait can be used by Account structure
pub struct Account {
    login: String,
    password: String,
}

///Implements Account structure associated functions
impl Account {
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

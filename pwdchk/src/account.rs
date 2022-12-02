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
}

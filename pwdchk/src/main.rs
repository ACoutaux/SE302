//! Main function module

mod account; //reference to account module
use account::*; //use impletentations of account module

fn main() {
    let account = Account::new("johndoe", "super:complex:password");
    println!("account = {account:#?}");
}
  
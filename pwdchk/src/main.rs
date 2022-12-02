//! Main function module

mod account; //reference to account module
use account::*; //use impletentations of account module

fn main() {
    println!("{:?}", Account::from_string("johndoe:super:complex:password"));
}
  
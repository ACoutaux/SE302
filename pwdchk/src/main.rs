//! Main function module

mod account; //reference to account module
use account::*; //use impletentations of account module
use std::str::FromStr;

fn main() -> Result<(), NoColon> {
    let v = std::env::args().skip(1).map(|x| Account::from_str(x.as_str())).collect::<Result<Vec<_>,_>>()?;
    println!("{:#?}",v);
    Ok(())
}
  
use super::account::*; //to use Account implementation from account module
use sha1::{Digest, Sha1};

pub fn sha1(account: &Account) -> (String, String) {

    // create a Sha1 object
    let mut hasher = Sha1::new();

    // process input message
    hasher.update(account.password.clone());

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();
    let hexa_result = format!("{result:X}");

    println!("{hexa_result:?}");

    (String::from('a'),String::from('a'))
}

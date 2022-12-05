use super::account::*; //to use Account implementation from account module
use sha1::{Digest, Sha1};

pub fn sha1(account: &Account) -> (String, String) {
    let mut hasher = Sha1::new(); // create a Sha1 object
    hasher.update(account.password.clone()); // process password

    let result = hasher.finalize(); // acquire hash digest in the form of GenericArray
   
    //Creates two mutable variables which will contain prefix and suffix of (capital) hexadeximal sha1    
    let mut hexa_result_prefix = format!("{result:X}"); 
    let mut hexa_result_suffix = format!("{result:X}");

    hexa_result_prefix.truncate(5);
    hexa_result_suffix.replace_range(0..5, "");

    (hexa_result_prefix, hexa_result_suffix) //returns prefix and suffix in a tuple
}

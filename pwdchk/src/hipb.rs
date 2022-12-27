use super::account::*; //to use Account implementation from account module
use super::error;
use error::Error;
use rayon::prelude::*;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
//use std::time::Instant; //to use time functions

fn sha1(account: &Account) -> (String, String) {
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

///Aplly in parallel sha1 function on accounts
fn all_sha1(accounts: &[Account]) -> Vec<(String, String, &Account)> {
    accounts
        .par_iter()
        .map(|x: &Account| {
            let y = sha1(x);
            (y.0, y.1, x)
        })
        .collect()
}

///Display execution time of all_sha1 function
/*pub fn all_sha1_timed(accounts: &[Account]) -> Vec<(String, String, &Account)> {
    let now = Instant::now(); //get current time before all_sha1 execution
    let sha1_tuple = all_sha1(accounts);
    let new_now = Instant::now(); //get current time after all_sha1 execution
    println!("{:?}", new_now.duration_since(now).as_micros()); //print execution time in us
    sha1_tuple
}*/

///Group sha1 with same prefix in a hash table with associated suffix and accounts
fn sha1_by_prefix(accounts: &[Account]) -> HashMap<String, Vec<(String, &Account)>> {
    let mut prefix_map: HashMap<String, Vec<(String, &Account)>> = HashMap::new(); //create new hash map
    let sha1_vec = all_sha1(accounts);
    for sha1 in sha1_vec.iter() {
        prefix_map
            .entry(sha1.0.clone())
            .and_modify(|e| e.push((sha1.1.clone(), sha1.2)))
            .or_insert(vec![(sha1.1.clone(), sha1.2)]);
    }
    prefix_map
}

///Returns lines in String vector or error of the submitted url
fn get_page(prefix: &str) -> Result<Vec<String>, Error> {
    let mut url: String = "https://api.pwnedpasswords.com/range/".to_string();
    url.push_str(prefix); //add prefix to previous url
    let response = reqwest::blocking::get(url).map_err(Error::from)?; //returns a reqwest error if get() method fails
    let suffixes = response.text()?;
    let suf_vec = suffixes.lines().map(|x| x.to_string()).collect();
    Ok(suf_vec)
}

///Returns hash map with suffixes and associated occurences linked to a prefix
fn get_suffixes(prefix: &str) -> Result<HashMap<String, u64>, Error> {
    let suffixes_vec = get_page(prefix)?;
    let mut suffixe_map: HashMap<String, u64> = HashMap::new(); //create new hash map for suffix and occurences
    suffixes_vec.iter().for_each(|x| {
        if !x.is_empty() {
            //to be sure to not take in account empty lines
            if x.contains(':') {
                let occ: u64 = u64::from_str_radix(x.split_once(':').unwrap().1, 32).unwrap(); //convert occurences number
                suffixe_map.insert(x.split_once(':').unwrap().0.to_string(), occ);
                //split suffixes and occurences in hash map
            }
        }
    });
    Ok(suffixe_map)
}

///Returns list of accounts with associated number of data leakages
pub fn check_accounts(accounts: &[Account]) -> Result<Vec<(&Account, u64)>, Error> {
    let mut result: Vec<(&Account, u64)> = vec![]; //init vector to receive output function result
    let prefix_hashmap = sha1_by_prefix(accounts); //get prefix hashmap from sha1_by_prefix function
    prefix_hashmap.iter().for_each(|(prefix, suff_accounts)| {
        let hacked_suff: HashMap<String, u64> = get_suffixes(prefix).unwrap(); //get suffix found in a leak for each prefix
        for suff_account in suff_accounts.iter() {
            //loop on accounts to be checked
            if hacked_suff.contains_key(&suff_account.0) {
                //check if each suffix is in the leaked suffixes hashmap
                result.push((suff_account.1, *hacked_suff.get(&suff_account.0).unwrap()));
            } else {
                result.push((suff_account.1, 0));
            }
        }
    });
    result.sort_unstable_by_key(|x| !x.1); //sort result by frequency of password leacks for each account (biggest frequencies first)
    Ok(result)
}

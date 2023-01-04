///This macro returns all possible combinations of vectors inputs
macro_rules! cartesian {
    //First option with 2 vectors input
    ($U:expr,$V:expr) => {{
        let u = $V.into_iter().collect::<Vec<_>>(); //collect the second member expressions in a vector in order to not compute values several times
        $U.into_iter().flat_map(move |l| u.clone().into_iter().map(move |r| (l.clone(),r))) //returns iterations of combinations of U and V
    }}
}

fn main() {
    let prod = cartesian!(
        [1, 2, 3],
        [String::from("foo"), String::from("bar")]
      ).collect::<Vec<_>>();
    println!("{prod:?}");
}
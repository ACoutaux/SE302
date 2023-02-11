mod executor;

#[cfg(test)]
mod test{
#[test]
fn it_works() {
    let res = async { 10 };
    assert_eq!(super::executor::executor::block_on(res), 10);
}
}


mod executor;

#[cfg(test)]
mod test{
#[test]
fn it_works() {
    let res = async { 10 };
    assert_eq!(super::executor::executor::block_on(res), 10);
}

#[test]
fn thread_works() {
        let rx = super::executor::executor::run_on_thread(async {
            let x = 10;
            x
        });
        assert_eq!(rx.recv().unwrap(), 10);
    }
}


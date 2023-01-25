pub mod executor {

    use std::sync::{Condvar,Mutex};
    use std::future::Future;

    struct CondVarWaker {
        condVar : Condvar,
        mutex : Mutex<bool>,
    }

    impl Default for CondVarWaker {
        fn default() -> Self {
            CondVarWaker { condVar: Condvar::new(), mutex : Mutex::new(false) } //mutex with bool initialised in false state
        }
    } 

    impl std::task::Wake for CondVarWaker {
        fn wake(self: std::sync::Arc<Self>) {}
    }

    impl CondVarWaker {
        fn wait_until_woken(&self) {
            let mut is_woken = self.mutex.lock().unwrap();
            is_woken = self.condVar.wait_while(is_woken, |x| !*x).unwrap();
            *is_woken = false; 
        }
    }

    pub fn block_on<T>(fut: impl Future<Output = T>) -> T {
        todo!()
    }
}
  
  
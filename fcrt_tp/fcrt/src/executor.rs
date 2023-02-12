pub mod executor {

    use std::sync::{Condvar,Mutex,Arc};
    use std::future::Future;
    use std::task::{Context,Poll,Waker};
    use std::sync::mpsc;
    use std::thread;
    use std::pin::Pin;

    struct CondVarWaker {
        cond_var : Condvar,
        mutex : Mutex<bool>,
    }

    impl Default for CondVarWaker {
        fn default() -> Self {
            Self { cond_var: Condvar::new(), mutex : Mutex::new(false) } //mutex with bool initialised in false state
        }
    } 

    impl std::task::Wake for CondVarWaker {
        fn wake(self: Arc<Self>) {
            let mut is_woken = self.mutex.lock().unwrap();
            *is_woken = true;
            self.cond_var.notify_one(); //wakes up one thread
        }
    
        //wake_by_ref takes reference on Arc<CondVarWaker>
        fn wake_by_ref(self: &Arc<Self>) {
            let mut is_woken = self.mutex.lock().unwrap();
            *is_woken = true;
            self.cond_var.notify_one(); //wakes up one thread
        }
    }

    impl CondVarWaker {
        fn wait_until_woken(&self) {
            let mut is_woken = self.mutex.lock().unwrap();
            is_woken = self.cond_var.wait_while(is_woken, |x| !*x).unwrap();
            *is_woken = false; //is_woken is reset to false after function execution
        }
    }

    pub fn block_on<T>(mut fut: impl Future<Output = T>) -> T {
        let arc_condvar = Arc::new(CondVarWaker::default());
        let waker_arc_condvar = arc_condvar.clone().into();
        let mut context = Context::from_waker(&waker_arc_condvar);
        loop {
            let fut = unsafe { Pin::new_unchecked(&mut fut) };
            match fut.poll(&mut context) {
                Poll::Pending => 
                    arc_condvar.wait_until_woken(),
                Poll::Ready(val) => 
                    return val,
            }
        }
    }

    pub fn run_on_thread<T: Send + 'static> (fut: impl Future<Output = T> + Send + 'static) 
    -> mpsc::Receiver<T> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = block_on(fut);
            tx.send(val).expect("error");
        });
        rx
    }

}
  
  
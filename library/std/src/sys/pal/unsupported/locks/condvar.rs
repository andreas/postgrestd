use crate::sys::sync::Mutex;
>>>>>>>> 19b41569e69ccc3111029ba7c53788f2cc4df06a:library/std/src/sys/sync/condvar/no_threads.rs
use crate::time::Duration;

pub struct Condvar {}

impl Condvar {
    #[inline]
    #[rustc_const_stable(feature = "const_locks", since = "1.63.0")]
    pub const fn new() -> Condvar {
        Condvar {}
    }

    #[inline]
    pub fn notify_one(&self) {}

    #[inline]
    pub fn notify_all(&self) {}

    pub unsafe fn wait(&self, _mutex: &Mutex) {
        panic!("condvar wait not supported")
    }

    pub unsafe fn wait_timeout(&self, _mutex: &Mutex, _dur: Duration) -> bool {
        panic!("condvar wait not supported");
    }
}

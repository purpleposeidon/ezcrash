/// Add additional information after a `panic`. Requires stack unwinding.
#[macro_export]
macro_rules! ctx {
    ($($tt:tt)*) => {
        let _ctx = $crate::Defer(|| {
            eprintln!($($tt)*);
        });
    };
}
#[doc(hidden)]
pub struct Defer<T: Fn()>(pub T);
impl<T: Fn()> Drop for Defer<T> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            (self.0)();
        }
    }
}

#[cfg(test)]
#[test]
#[should_panic]
fn panic_ctx() {
    let val = format!("test value");
    ctx!("val = {val:?}");
    panic!("oh no: {}", val);
}

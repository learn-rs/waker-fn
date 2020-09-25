use waker_fn::waker_fn;

fn main() {
    let waker = waker_fn(||println!("woken"));
    waker.wake_by_ref();
    waker.wake();
}
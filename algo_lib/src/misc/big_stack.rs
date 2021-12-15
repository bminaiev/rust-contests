use std::thread;

pub fn with_set_stack<F, T>(size: usize, f: F) -> T
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    thread::Builder::new()
        .stack_size(size)
        .spawn(f)
        .unwrap()
        .join()
        .unwrap()
}

pub fn with_big_stack<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    with_set_stack(256_000_000, f)
}

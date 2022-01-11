pub trait LastExn<T> {
    fn last_exn(&self) -> &T;
}

impl<T> LastExn<T> for &[T] {
    fn last_exn(&self) -> &T {
        self.last().unwrap()
    }
}

impl<T> LastExn<T> for Vec<T> {
    fn last_exn(&self) -> &T {
        self.last().unwrap()
    }
}

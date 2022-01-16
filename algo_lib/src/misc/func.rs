pub fn id<T>(x: T) -> T {
    x
}

pub fn fst<T, U>((x, y): (T, U)) -> T {
    x
}

pub fn snd<T, U>((x, y): (T, U)) -> U {
    y
}

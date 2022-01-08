pub fn digits(number: i64) -> Vec<i32> {
    digits_base(number, 10)
}

pub fn digits_base(mut number: i64, base: i64) -> Vec<i32> {
    let mut res = vec![];
    while number != 0 {
        res.push((number % base) as i32);
        number /= base;
    }
    res.reverse();
    res
}

pub fn digit_from_char(c: u8) -> i32 {
    assert!(c >= b'0');
    assert!(c <= b'9');
    (c - b'0') as i32
}

pub fn char_from_digit(digit: i32) -> u8 {
    assert!(digit >= 0);
    assert!(digit <= 9);
    b'0' + digit as u8
}

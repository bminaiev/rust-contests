use std::sync::atomic::{AtomicUsize, Ordering};

crate::avx2! {
    fn update(dst: &mut [i32], src: &[i32], delta: i32) -> i32 {
        assert_eq!(dst.len(), src.len());
        let mut best = i32::MIN;
        for i in 0..dst.len() {
            dst[i] = dst[i].max(src[i] + delta);
            best = best.max(dst[i]);
        }
        best
    }
}

crate::avx2! {
    pub(crate) fn first(a: &[i32]) -> Option<&i32> {
        a.first()
    }
}

crate::avx2! {
    fn owned(value: String) -> String {
        value + "!"
    }
}

crate::avx2! {
    fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
        let value = s.parse::<i32>()?;
        if value < 0 {
            return Ok(0);
        }
        Ok(value)
    }
}

static CALLS: AtomicUsize = AtomicUsize::new(0);

crate::avx2! {
    fn main() {
        CALLS.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn slices_and_vector_tails() {
    for len in 0..=129 {
        let src: Vec<_> = (0..len).map(|i| i as i32 * 13 - 500).collect();
        let mut dst: Vec<_> = (0..len).map(|i| 400 - i as i32 * 7).collect();
        let expected: Vec<_> = dst.iter().zip(&src).map(|(&a, &b)| a.max(b + 29)).collect();
        let best = update(&mut dst, &src, 29);
        assert_eq!(dst, expected);
        assert_eq!(best, expected.iter().copied().max().unwrap_or(i32::MIN));
    }
}

#[test]
fn borrowed_return() {
    let a = [7, 9];
    assert!(std::ptr::eq(first(&a).unwrap(), &a[0]));
    assert_eq!(first(&[]), None);
}

#[test]
fn owned_argument_and_return() {
    assert_eq!(owned(String::from("hello")), "hello!");
}

#[test]
fn returns_and_question_mark() {
    assert_eq!(parse("17"), Ok(17));
    assert_eq!(parse("-3"), Ok(0));
    assert!(parse("bad").is_err());
}

#[test]
fn main_body_runs_once() {
    main();
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);
}

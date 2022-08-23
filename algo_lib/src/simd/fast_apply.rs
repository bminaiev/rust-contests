#[target_feature(enable = "avx2")]
unsafe fn fast_apply_avx2<T: Copy, F>(a: &mut [T], mut f: F)
where
    F: FnMut(T) -> T,
{
    for val in a.iter_mut() {
        *val = f(*val);
    }
}

pub fn fast_apply<T: Copy, F>(a: &mut [T], f: F)
where
    F: FnMut(T) -> T,
{
    unsafe { fast_apply_avx2(a, f) }
}

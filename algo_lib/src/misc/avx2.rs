/// Compile a function body for AVX2 and for the baseline CPU, selecting at runtime.
///
/// ```
/// algo_lib::avx2! {
///     fn sum(a: &[i32]) -> i32 {
///         a.iter().sum()
///     }
/// }
/// assert_eq!(sum(&[1, 2, 3]), 6);
/// ```
///
/// Accepts non-generic free functions with named parameters, including `fn main()`.
/// Only this function's body is specialized: separately compiled callees do not
/// inherit AVX2. Wrap hot callees too if they are not inlined into this function.
/// Keep CPU-specific intrinsics out of the body, which must also work without AVX2.
#[macro_export]
macro_rules! avx2 {
    (
        $(#[$attr:meta])*
        $vis:vis fn $name:ident($($arg:ident: $ty:ty),* $(,)?) $(-> $ret:ty)?
        $body:block
    ) => {
        $(#[$attr])*
        $vis fn $name($($arg: $ty),*) $(-> $ret)? {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            #[target_feature(enable = "avx2")]
            #[deny(unsafe_op_in_unsafe_fn)]
            unsafe fn __avx2($($arg: $ty),*) $(-> $ret)? $body

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            if std::arch::is_x86_feature_detected!("avx2") {
                // SAFETY: the runtime check includes CPU and OS support for AVX2.
                return unsafe { __avx2($($arg),*) };
            }

            $body
        }
    };
}

pub fn print_available_sse_extensions() {
    if is_x86_feature_detected!("sse2") {
        println!("Runtime available: sse2");
    }
    if is_x86_feature_detected!("sse3") {
        println!("Runtime available: sse3");
    }
    if is_x86_feature_detected!("ssse3") {
        println!("Runtime available: ssse3");
    }
    if is_x86_feature_detected!("sse4.1") {
        println!("Runtime available: sse4.1");
    }
    if is_x86_feature_detected!("avx2") {
        println!("Runtime available: avx2");
    }
    #[cfg(target_feature = "sse2")]
    println!("Compile time available: sse2");
    #[cfg(target_feature = "sse3")]
    println!("Compile time available: sse3");
    #[cfg(target_feature = "ssse3")]
    println!("Compile time available: ssse3");
    #[cfg(target_feature = "sse4.1")]
    println!("Compile time available: sse4.1");
    #[cfg(target_feature = "avx2")]
    println!("Compile time available: avx2");
}

pub fn z_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        if i <= r {
            z[i] = std::cmp::min(r - i + 1, z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

pub fn gen_primes_table(up_to: usize) -> Vec<bool> {
    let mut is_prime = vec![true; up_to + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..is_prime.len() {
        if is_prime[p] {
            for another in (2 * p..is_prime.len()).step_by(p) {
                is_prime[another] = false;
            }
        }
    }
    is_prime
}

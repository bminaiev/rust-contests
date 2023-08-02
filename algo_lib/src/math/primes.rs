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

pub fn gen_largest_prime_table(up_to: usize) -> Vec<usize> {
    let mut largest_prime = vec![0; up_to + 1];
    for p in 2..largest_prime.len() {
        if largest_prime[p] == 0 {
            for another in (p..largest_prime.len()).step_by(p) {
                largest_prime[another] = p;
            }
        }
    }
    largest_prime
}

pub struct PrimesIter<'a> {
    largest_primes: &'a [usize],
    value: usize,
}

pub struct Prime {
    pub value: usize,
    pub power: usize,
}

pub fn factorize(largest_primes: &[usize], value: usize) -> PrimesIter {
    PrimesIter {
        largest_primes,
        value,
    }
}

impl<'a> Iterator for PrimesIter<'a> {
    type Item = Prime;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 1 {
            None
        } else {
            let prime = self.largest_primes[self.value];
            let mut power = 0;
            while self.value % prime == 0 {
                self.value /= prime;
                power += 1;
            }
            Some(Prime {
                value: prime,
                power,
            })
        }
    }
}

pub fn is_prime(x: i64) -> bool {
    if x <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

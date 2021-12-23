use crate::misc::rand::Random;
use crate::seg_trees::fenwick::Fenwick;

#[test]
fn stress() {
    let mut rnd = Random::new(787788);
    const MAX_N: usize = 100;
    const MAX_VAL: i32 = i32::MAX;
    const TESTS_N: usize = 100;

    for _ in 0..TESTS_N {
        let n: usize = rnd.next_in_range(1, MAX_N);
        let mut fenw = Fenwick::new(n);
        let mut slow_vec = vec![0i64; n];
        for _ in 0..TESTS_N {
            let pos = rnd.next_in_range(0, n);
            if rnd.next_double() < 0.5 {
                let sum_from_fenw = fenw.get_sum(pos);
                let sum_slow = slow_vec[0..=pos].iter().sum();
                assert_eq!(sum_from_fenw, sum_slow);
            } else {
                let change = rnd.next_in_range(0, MAX_VAL as usize) as i64;
                fenw.add(pos, change);
                slow_vec[pos] += change;
            }
        }
    }
}

#[test]
fn stress_speed() {
    const MAX_N: usize = 1_000_000;
    const MAX_VAL: i32 = 1_000_000;
    const TESTS_N: usize = 1;
    const OPS_IN_TEST: usize = 20_000_000;

    for t in 0..TESTS_N {
        let mut rnd = Random::new((787788 + t) as u64);
        let now = std::time::Instant::now();
        let n: usize = MAX_N;
        let mut tree = Fenwick::new(n);
        let mut tot_sum = 0;
        for _ in 0..OPS_IN_TEST {
            let pos = rnd.next_in_range(0, n);
            if rnd.next_double() < 0.5 {
                tot_sum += tree.get_sum(pos);
            } else {
                let change = rnd.next_in_range(0, MAX_VAL as usize) as i64;
                tree.add(pos, change);
            }
        }
        eprintln!("hash val: {}", tot_sum);
        eprintln!("done with test in {}ms", now.elapsed().as_millis());
    }
}

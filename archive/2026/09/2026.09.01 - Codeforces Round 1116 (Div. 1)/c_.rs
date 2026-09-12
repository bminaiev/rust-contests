#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::gcd::{gcd, mod_inv};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;

fn solve_first(s: &[Vec<u8>], need_r: usize, need_c: usize) -> (usize, usize, usize, usize) {
    let n = s.len();
    let mut cnt_black = 0;
    let mut sum_r = 0;
    let mut sum_c = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == b'#' {
                cnt_black += 1;
                sum_r += i;
                sum_c += j;
            }
        }
    }
    let cnt_black_inv = mod_inv(cnt_black as i64, n as i64).unwrap() as usize;
    assert!((cnt_black * cnt_black_inv) % n == 1);
    sum_r %= n;
    sum_c %= n;
    let expected_sum_r = (need_r * cnt_black) % n;
    let expected_sum_c = (need_c * cnt_black) % n;
    let extra_shift_r = (expected_sum_r + n - sum_r) % n;
    let extra_shift_c = (expected_sum_c + n - sum_c) % n;
    if extra_shift_r == 0 && extra_shift_c == 0 {
        return (0, 0, 0, 0);
    }
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == b'#' {
                let ni = (i + extra_shift_r) % n;
                let nj = (j + extra_shift_c) % n;
                if s[ni][nj] != b'#' {
                    return (i, j, ni, nj);
                }
            }
        }
    }
    dbg!(extra_shift_r, extra_shift_c);
    unreachable!();
}

fn solve_second(s: &[Vec<u8>]) -> (usize, usize) {
    let n = s.len();
    let mut cnt_black = 0;
    let mut sum_r = 0;
    let mut sum_c = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == b'#' {
                cnt_black += 1;
                sum_r += i;
                sum_c += j;
            }
        }
    }
    let cnt_black_inv = mod_inv(cnt_black as i64, n as i64).unwrap() as usize;
    assert!((cnt_black * cnt_black_inv) % n == 1);
    sum_r %= n;
    sum_c %= n;
    let expected_r = (sum_r * cnt_black_inv) % n;
    let expected_c = (sum_c * cnt_black_inv) % n;
    (expected_r, expected_c)
}

fn solve(input: &mut Input, out: &mut Output) {
    let first = input.string()[0] == b'f';
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let s = gen_vec(n, |_| input.string());

        if first {
            let need_r = input.usize() - 1;
            let need_c = input.usize() - 1;
            let (r1, c1, r2, c2) = solve_first(&s, need_r, need_c);
            out.println(vec![r1 + 1, c1 + 1, r2 + 1, c2 + 1]);
        } else {
            let (expected_r, expected_c) = solve_second(&s);
            out.println((expected_r + 1, expected_c + 1));
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(2..10);
        let mut s = vec![vec![b'.'; n]; n];
        let prob = rnd.gen_double();
        let mut cnt_black = 0;
        for i in 0..n {
            for j in 0..n {
                if rnd.gen_double() < prob {
                    s[i][j] = b'#';
                    cnt_black += 1;
                }
            }
        }
        if gcd(n, cnt_black) != 1 {
            continue;
        }
        let need_r = rnd.gen_range(0..n);
        let need_c = rnd.gen_range(0..n);
        let (r1, c1, r2, c2) = solve_first(&s, need_r, need_c);
        let tmp = s[r1][c1];
        s[r1][c1] = s[r2][c2];
        s[r2][c2] = tmp;
        let (expected_r, expected_c) = solve_second(&s);
        assert_eq!(expected_r, need_r);
        assert_eq!(expected_c, need_c);
    }
}

fn stress123() {
    for n in 2..10 {
        for w in 0..n * n {
            if gcd(n, w) == 1 {
                let cnt_black = w;
                let cnt_black_inv = mod_inv(cnt_black as i64, n as i64).unwrap() as usize;
                dbg!(n, w, cnt_black_inv);
                assert!((cnt_black * cnt_black_inv) % n == 1);
            }
        }
    }
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}

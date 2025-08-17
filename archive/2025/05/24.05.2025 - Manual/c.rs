//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let mut a = Array2D::new(0, n, n);
    let mut a_rev = Array2D::new(0, n, n);
    for i in 0..n {
        for j in 0..n {
            let s = input.string_as_string();
            a[i][j] = s.parse::<i64>().unwrap();
            let s_rev = s.chars().rev().collect::<String>();
            a_rev[i][j] = s_rev.parse::<i64>().unwrap();
        }
    }
    let mut all_primes = vec![];
    {
        let mut x = a[0][0];
        for it in 2.. {
            if it * it > x {
                break;
            }
            if x % it == 0 {
                all_primes.push(it);
                while x % it == 0 {
                    x /= it;
                }
            }
        }
        if x > 1 {
            all_primes.push(x);
        }
        x = a_rev[0][0];
        for it in 2.. {
            if it * it > x {
                break;
            }
            if x % it == 0 {
                all_primes.push(it);
                while x % it == 0 {
                    x /= it;
                }
            }
        }
        if x > 1 {
            all_primes.push(x);
        }
        all_primes.sort();
        all_primes.dedup();
    }
    let mut op_type = vec![];
    let mut ops_values = vec![];
    for _ in 0..q {
        let s = input.string();
        if s[0] == b'+' {
            op_type.push(1);
        } else {
            op_type.push(0);
        }
        let x = input.i32() + n as i32;
        ops_values.push(x);
    }
    let mut res = vec![1; q];
    let mut rng = Random::new(2345345);
    for &p in all_primes.iter() {
        let mut cur_pw = 1;
        loop {
            cur_pw *= p;
            let mut bad = false;
            let mut xors = vec![vec![0; 3 * n + 10]; 2];
            let mut now_xor = 0;
            for i in 0..n {
                for j in 0..n {
                    let ok_now = a[i][j] % cur_pw == 0;
                    let ok_rev = a_rev[i][j] % cur_pw == 0;
                    if !ok_now && !ok_rev {
                        bad = true;
                        break;
                    }
                    if ok_now && ok_rev {
                        continue;
                    }
                    let magic = rng.gen_u64();
                    {
                        let val_sum = i + j + 2 + n;
                        xors[1][val_sum] ^= magic;
                    }
                    {
                        let val_sub = i + n - j;
                        xors[0][val_sub] ^= magic;
                    }
                    if ok_rev {
                        now_xor ^= magic;
                    }
                }
            }
            if bad {
                break;
            }
            for i in 0..q {
                now_xor ^= xors[op_type[i]][ops_values[i] as usize];
                if now_xor == 0 {
                    res[i] *= p;
                }
            }
        }
    }
    for &x in res.iter() {
        out.println(x);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

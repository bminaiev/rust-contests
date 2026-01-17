//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

const K: usize = 9;
fn solve_fast(a: &[usize], c: i64) -> i64 {
    let n = a.len();
    let mut dp = vec![i64::MAX / 10; 1 << (K * 2)];
    let mut res = c * (n as i64);
    let mut cost_offset = 0;
    for i in 0..n {
        let value = a[i];
        let mut my_cost = cost_offset + (value as i64);
        for mask in 0..(1 << K) {
            let look_at = value ^ mask;
            let prev_cost = dp[look_at] + cost_offset + (mask) as i64;
            if prev_cost < my_cost {
                my_cost = prev_cost;
            }
        }
        // dbg!(i, my_cost, value);
        {
            // if final
            let more = (n - i - 1) as i64;
            let total_cost = my_cost + value as i64 + more * c;
            // dbg!(value, total_cost, more);
            if total_cost < res {
                res = total_cost;
            }
        }
        cost_offset += c;
        for high_mask in 0..(1 << K) {
            let put_to = value ^ (high_mask << K);
            let new_cost = my_cost + (high_mask << K) as i64 - cost_offset;
            if new_cost < dp[put_to] {
                dp[put_to] = new_cost;
            }
        }
    }
    res
}

fn solve_slow(a: &[usize], c: i64) -> i64 {
    let n = a.len();
    let mut res = i64::MAX;
    for mask in 0..(1 << a.len()) {
        let mut b = vec![];
        let mut cost = 0;
        for i in 0..n {
            if ((1 << i) & mask) != 0 {
                b.push(a[i]);
            } else {
                cost += c;
            }
        }
        if b.len() > 0 {
            cost += b[0] as i64;
            cost += b[b.len() - 1] as i64;
            for i in 1..b.len() {
                cost += (b[i] ^ b[i - 1]) as i64;
            }
        }
        if cost < res {
            res = cost;
        }
    }
    res
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(2..10);
        let a = rnd.gen_vec(n, 1..n);
        let c = rnd.gen_range(0..10);
        let slow = solve_slow(&a, c);
        let fast = solve_fast(&a, c);
        if slow != fast {
            dbg!(a, c, slow, fast);
            panic!();
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let c = input.i64();

        let a = input.vec::<usize>(n);
        let res = solve_fast(&a, c);
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "i";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}

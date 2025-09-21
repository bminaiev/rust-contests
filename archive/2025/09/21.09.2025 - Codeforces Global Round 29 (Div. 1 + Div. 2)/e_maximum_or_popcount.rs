//{"name":"E. Maximum OR Popcount","group":"Codeforces - Codeforces Global Round 29 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2147/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 3\n0\n0\n2\n4\n2 2\n1 3\n0\n3\n2 1\n1000000000 1000000000\n1000000000\n","output":"0\n1\n2\n2\n3\n31\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let q = input.usize();
        let a = input.vec::<i64>(n);
        let mut cur_or = 0;
        for &x in a.iter() {
            cur_or |= x;
        }
        const BITS: usize = 40;
        let mut sort_by = vec![vec![]; BITS];
        for cnt_bits in 1..BITS {
            sort_by[cnt_bits] = (0..n).collect();
            sort_by[cnt_bits].sort_by_key(|&i| -(a[i] & ((1 << cnt_bits) - 1)));
        }
        let mut cost = vec![i64::MAX; BITS];
        cost[cur_or.count_ones() as usize] = 0;
        for cnt_bits in 1..BITS {
            let expected_value = cur_or | ((1 << cnt_bits) - 1);
            let mut expected_cost = 0;

            let mut used = vec![false; n];
            for bit in (0..cnt_bits).rev() {
                let mut already = false;
                for i in 0..n {
                    if a[i] & (1 << bit) != 0 && !used[i] {
                        already = true;
                        break;
                    }
                }
                if already {
                    continue;
                }
                let mut it = 0;
                let mut found = false;
                while it < sort_by[bit + 1].len() {
                    let i = sort_by[bit + 1][it];
                    if !used[i] {
                        used[i] = true;
                        let now = a[i] & ((1 << (bit + 1)) - 1);
                        assert!(now < (1 << bit));
                        let more = (1 << bit) - now;
                        expected_cost += more;
                        found = true;
                        break;
                    }
                    it += 1;
                }
                if !found {
                    expected_cost += 1 << bit;
                }
            }

            let cbits = expected_value.count_ones() as usize;
            if cost[cbits] > expected_cost {
                cost[cbits] = expected_cost;
            }
        }
        for i in (1..BITS).rev() {
            if cost[i - 1] > cost[i] {
                cost[i - 1] = cost[i];
            }
        }
        for _ in 0..q {
            let can_spend = input.i64();
            let mut it = 0;
            while cost[it + 1] <= can_spend {
                it += 1;
            }
            out.println(it);
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e_maximum_or_popcount";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
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

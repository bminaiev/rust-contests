//{"name":"D - Devourers and Cake","group":"AtCoder - AtCoder Regular Contest 207 (Div.1)","url":"https://atcoder.jp/contests/arc207/tasks/arc207_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 2\n01\n10\n1 4\n0100\n4 1\n0\n1\n0\n0\n5 5\n00000\n11100\n01011\n00100\n01101\n","output":"Second\nFirst\nFirst\nSecond\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::HashMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable5, RecursiveFunction5};

fn solve_single(a: &[Vec<u8>], min_sz: usize) -> bool {
    let mut cache = HashMap::new();
    let mut gen_ans = RecursiveFunction5::new(
        |f, x_from: usize, x_to: usize, y_from: usize, y_to: usize, want: bool| -> bool {
            let cache_entry = (x_from, x_to, y_from, y_to, want);
            if let Some(&res) = cache.get(&cache_entry) {
                return res;
            }
            let dx = x_to - x_from;
            let dy = y_to - y_from;
            if dx > min_sz {
                return f.call(x_from + 1, x_to - 1, y_from, y_to, !want);
            }
            if dy > min_sz {
                return f.call(x_from, x_to, y_from + 1, y_to - 1, !want);
            }
            if dx == 1 && dy == 1 {
                let want_char = if want { b'1' } else { b'0' };
                return a[x_from][y_from] == want_char;
            }
            let res = {
                if dx > 1 && !f.call(x_from + 1, x_to, y_from, y_to, !want) {
                    true
                } else if dx > 1 && !f.call(x_from, x_to - 1, y_from, y_to, !want) {
                    true
                } else if dy > 1 && !f.call(x_from, x_to, y_from + 1, y_to, !want) {
                    true
                } else if dy > 1 && !f.call(x_from, x_to, y_from, y_to - 1, !want) {
                    true
                } else {
                    false
                }
            };
            cache.insert(cache_entry, res);
            res
        },
    );
    let ans = gen_ans.call(0, a.len(), 0, a[0].len(), true);
    ans
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let mut a = vec![];
        for _ in 0..n {
            a.push(input.string());
        }
        let ans = solve_single(&a, 3);
        if ans {
            out.println("First");
        } else {
            out.println("Second");
        }
    }
}

fn stress() {
    for it in 2.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MX: usize = 10;
        let n = rnd.gen_range(1..MX);
        let m = rnd.gen_range(1..MX);
        let mut a = vec![];
        for _ in 0..n {
            let mut row = vec![];
            for _ in 0..m {
                row.push(if rnd.gen_bool() { b'0' } else { b'1' });
            }
            a.push(row);
        }
        let ans1 = solve_single(&a, 3);
        let ans2 = solve_single(&a, MX);
        dbg!(n, m, ans1, ans2);
        if ans1 != ans2 {
            for row in a {
                dbg!(String::from_utf8(row).unwrap());
            }
            dbg!(3, ans1);
            dbg!(4, ans2);
            dbg!(n + m);
            panic!();
        }
        assert_eq!(ans1, ans2);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "d_devourers_and_cake";
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

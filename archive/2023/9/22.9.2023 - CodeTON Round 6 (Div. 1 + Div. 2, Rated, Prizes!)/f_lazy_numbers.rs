//{"name":"F. Lazy Numbers","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"8\n2 2\n4 2\n6 4\n33 2\n532 13\n780011804570805480 3788\n366364720306464627 4702032149561577\n293940402103595405 2\n","output":"2\n2\n1\n3\n1\n3789\n1\n7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLazyNumbers"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn conv_to_vec(mut x: i64, base: i64) -> Vec<i64> {
    let mut res = vec![];
    while x != 0 {
        res.push(x % base);
        x /= base;
    }
    res.reverse();
    res
}

fn find_pref_n(same_prefix: usize, n_conv: &[i64], base: i64, ways_to_finish: &[i64]) -> i64 {
    let max_len = n_conv.len();
    let mut res = 0;
    for same_prefix_n in same_prefix + 1..max_len {
        let mut smaller_ways = n_conv[same_prefix_n];
        if same_prefix_n == 0 {
            smaller_ways -= 1;
        }
        res += smaller_ways * ways_to_finish[max_len - same_prefix_n - 1];
        if same_prefix_n > same_prefix {
            // just use first [same_prefix_n] digits of [N]
            res += 1;
        }
        if same_prefix_n != max_len - 1 {
            let mut max_here = base - 1;
            res += (max_here - n_conv[same_prefix_n]) * ways_to_finish[max_len - same_prefix_n - 2];
        }
    }
    // exactly n
    res += 1;
    res
}

fn find_pos(x: i64, base: i64, n_conv: &[i64], cache: &[i64], ways_to_finish: &[i64]) -> i64 {
    let max_len = n_conv.len();
    let x_conv = conv_to_vec(x, base);
    let mut res = 1;

    // max_len = 3
    // [1, 2, 3]
    // same_prefix = 2
    let mut cmp_n = 0;
    for same_prefix in 0..x_conv.len() {
        let possible_first = if same_prefix == 0 {
            x_conv[0] - 1
        } else {
            x_conv[same_prefix]
        };
        let mut more_len = max_len - same_prefix - 1;
        if cmp_n == 1 {
            more_len -= 1;
        }
        if same_prefix != 0 {
            // just use first [same_prefix] digits of [X]
            res += 1;
        }
        if cmp_n != 0 || n_conv[same_prefix] >= x_conv[same_prefix] {
            res += possible_first * ways_to_finish[more_len];
        } else {
            let same_prefix_n = same_prefix;
            let mut smaller_ways = n_conv[same_prefix_n];
            if same_prefix_n == 0 {
                smaller_ways -= 1;
            }
            res += smaller_ways * ways_to_finish[max_len - same_prefix_n - 1];
            if same_prefix_n > same_prefix {
                // just use first [same_prefix_n] digits of [N]
                res += 1;
            }
            if same_prefix_n != max_len - 1 {
                let mut max_here = base - 1;
                if same_prefix_n == same_prefix {
                    max_here = x_conv[same_prefix_n] - 1;
                }
                res += (max_here - n_conv[same_prefix_n])
                    * ways_to_finish[max_len - same_prefix_n - 2];
            }
            res += cache[same_prefix];
        }
        if cmp_n == 0 {
            if x_conv[same_prefix] < n_conv[same_prefix] {
                cmp_n = -1;
            } else if x_conv[same_prefix] > n_conv[same_prefix] {
                cmp_n = 1;
            }
        }
    }
    res
}

fn solve_one(n: i64, base: i64) -> i64 {
    let mut res = 0;
    let n_conv = conv_to_vec(n, base);
    let max_len = n_conv.len();

    let mut ways_to_finish = vec![1; max_len];
    let mut pw = 1;
    for i in 1..ways_to_finish.len() {
        pw *= base;
        ways_to_finish[i] = ways_to_finish[i - 1] + pw;
    }
    let cache = gen_vec(max_len, |same_prefix| {
        find_pref_n(same_prefix, &n_conv, base, &ways_to_finish)
    });

    for len in 1..=max_len {
        let mut from = 1;
        let mut to = base - 1;
        for _ in 0..len - 1 {
            from *= base;
            to = to * base + (base - 1);
        }
        to = min(to, n);
        let ok_from = binary_search_first_true(from..to + 1, |x| {
            find_pos(x, base, &n_conv, &cache, &ways_to_finish) >= x
        });
        let ok_till = binary_search_first_true(from..to + 1, |x| {
            find_pos(x, base, &n_conv, &cache, &ways_to_finish) > x
        });
        res += ok_till - ok_from;
    }
    res
}

fn stress() {
    for _t in 0..1000 {
        let n = 10i64.pow(18);
        let base = 2;
        let res = solve_one(n, base);
        dbg!(_t, res);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i64();
    let base = input.i64();
    let res = solve_one(n, base);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN

//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::primes::is_prime;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn check(a: &[usize], n: usize) -> bool {
    let mx = a.iter().copied().max().unwrap();
    assert!(mx <= n);
    let mut used = vec![false; mx * 2 + 1];
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            let sum = a[i] + a[j];
            if used[sum] {
                return false;
            }
            used[sum] = true;
        }
    }
    true
}

fn stress() {
    for n in 10..=1000 {
        dbg!(n);
        solve_case(n);
    }
}

fn solve_case(n: usize) -> Vec<usize> {
    let need_numbers = ((n as f64).sqrt() / 2.0).ceil() as usize;
    if need_numbers == 1 {
        assert!(n >= 1);
        return vec![1];
    }
    if need_numbers == 2 {
        assert!(n >= 2);
        return vec![1, 2];
    }
    // i -> 2 * i * prime + (i ^ 2 % prime)
    // prime >= need_numbers
    let mut p = need_numbers;
    while !is_prime(p as i64) {
        p += 1;
    }
    let a: Vec<_> = (0..need_numbers)
        .map(|i| 1 + 2 * i * p + (i * i % p))
        .collect();
    assert!(check(&a, n));
    a
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = solve_case(n);
    out_line!(a.len());
    out_line!(a);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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

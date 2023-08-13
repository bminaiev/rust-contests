//{"name":"A - i i's","group":"AtCoder - AtCoder Grand Contest 064","url":"https://atcoder.jp/contests/agc064/tasks/agc064_a","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n","output":"1 3 4 2 4 3 4 2 4 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AIIs"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_case(n: usize) -> Vec<usize> {
    assert!(n >= 3);
    let mut res = vec![];
    let mut more = gen_vec(n + 1, |x| x);
    more[n - 2] -= 1;

    let mut go = RecursiveFunction::new(|f, cur: usize| -> bool {
        if more[cur] == 0 {
            return false;
        }
        res.push(cur);
        more[cur] -= 1;
        if cur > 2 {
            f.call(cur - 2);
        }
        if more[cur - 1] > 0 {
            res.push(cur - 1);
            more[cur - 1] -= 1;
        }
        true
    });
    loop {
        if !go.call(n) {
            break;
        }
    }

    res.push(n - 2);
    for &x in more.iter() {
        assert!(x == 0);
    }
    for i in 0..res.len() {
        let delta = res[i].abs_diff(res[(i + 1) % res.len()]);
        assert!(delta == 1 || delta == 2);
    }
    res
}

fn stress() {
    let n = 5;
    for n in 3..=100 {
        let res = solve_case(n);
        dbg!(n, res);
    }
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let res = solve_case(n);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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

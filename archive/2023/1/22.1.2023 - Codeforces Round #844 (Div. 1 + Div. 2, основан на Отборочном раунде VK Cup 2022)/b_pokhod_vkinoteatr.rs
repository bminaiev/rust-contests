//{"name":"B. Поход в кинотеатр","group":"Codeforces - Codeforces Round #844 (Div. 1 + Div. 2, основан на Отборочном раунде VK Cup 2022)","url":"https://codeforces.com/contest/1782/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 1\n7\n0 1 2 3 4 5 6\n8\n6 0 3 3 6 7 2 7\n5\n3 0 0 3 3\n","output":"2\n1\n3\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPokhodVKinoteatr"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<usize>(n);
    a.sort();
    let mut res = 0;
    let mut it = 0;
    while it <= a.len() {
        while it < a.len() && a[it] <= it {
            it += 1;
        }
        if it == 0 || (it - 1) >= a[it - 1] {
            res += 1;
        }
        it += 1;
    }
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN

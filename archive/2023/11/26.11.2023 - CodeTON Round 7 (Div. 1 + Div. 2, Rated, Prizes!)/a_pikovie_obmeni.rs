//{"name":"A. Пиковые обмены","group":"Codeforces - CodeTON Round 7 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1896/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3\n1 2 3\n5\n1 3 2 5 4\n5\n5 4 3 2 1\n3\n3 1 2\n4\n2 3 1 4\n5\n5 1 2 3 4\n","output":"YES\nYES\nNO\nNO\nNO\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APikovieObmeni"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i32>(n);
    loop {
        let mut ch = false;
        for i in 1..(n - 1) {
            if a[i] > a[i - 1] && a[i] > a[i + 1] {
                a.swap(i, i + 1);
                ch = true;
            }
        }
        if !ch {
            break;
        }
    }
    let mut ok = true;
    for i in 0..(n - 1) {
        if a[i] > a[i + 1] {
            ok = false;
        }
    }
    if ok {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
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

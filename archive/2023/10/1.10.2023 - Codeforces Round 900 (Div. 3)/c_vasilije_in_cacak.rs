//{"name":"C. Vasilije in Cacak","group":"Codeforces - Codeforces Round 900 (Div. 3)","url":"https://codeforces.com/contest/1878/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"12\n5 3 10\n5 3 3\n10 10 55\n6 5 20\n2 1 26\n187856 87856 2609202300\n200000 190000 19000000000\n28 5 2004\n2 2 2006\n9 6 40\n47202 32455 613407217\n185977 145541 15770805980\n","output":"YES\nNO\nYES\nYES\nNO\nNO\nYES\nNO\nNO\nNO\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVasilijeInCacak"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i64();
    let k = input.i64();
    let x = input.i64();
    let min_sum = k * (k + 1) / 2;
    let max_sum = min_sum + k * (n - k);
    if x >= min_sum && x <= max_sum {
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

//{"name":"A. МЕХанизированный массив","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n5 3 3\n4 7 5\n4 2 28\n12 10 6\n57 51 122\n200 1 200\n2 2 1\n3 2 1\n4 7 10\n","output":"7\n-1\n57\n-1\n2007\n39800\n1\n2\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMEKhanizirovanniiMassiv"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i32();
    let k = input.i32();
    let x = input.i32();
    if k > n || k > x + 1 {
        out_line!(-1);
    } else {
        let mut sum = 0;
        for i in 0..k {
            sum += i;
        }
        for i in k..n {
            sum += if k == x { k - 1 } else { x };
        }
        out_line!(sum);
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

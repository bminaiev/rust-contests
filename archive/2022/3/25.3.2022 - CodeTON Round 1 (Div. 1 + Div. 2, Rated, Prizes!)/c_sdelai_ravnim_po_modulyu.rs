//{"name":"C. Сделай равным по модулю","group":"Codeforces - CodeTON Round 1 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1656/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n2 5 6 8\n3\n1 1 1\n5\n4 1 7 0 8\n4\n5 9 17 5\n","output":"YES\nYES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSdelaiRavnimPoModulyu"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i32>(n);
    a.sort();
    let mut exist1 = false;
    for &x in a.iter() {
        if x == 1 {
            exist1 = true;
            break;
        }
    }
    if !exist1 {
        out_line!("YES");
        return;
    }
    let mut close_pair = false;
    for w in a.windows(2) {
        if w[0] + 1 == w[1] {
            close_pair = true;
            break;
        }
    }
    if close_pair {
        out_line!("NO");
    } else {
        out_line!("YES");
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
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
}
//END MAIN

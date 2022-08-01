//{"name":"A. Две 0-1 последовательности","group":"Codeforces - CodeTON Round 2 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1704/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n6 2\n001001\n11\n6 2\n110111\n01\n6 2\n000001\n11\n6 2\n111111\n01\n8 5\n10000101\n11010\n7 4\n1010001\n1001\n8 6\n01010010\n010010\n8 4\n01010101\n1001\n8 4\n10101010\n0110\n7 5\n1011100\n11100\n","output":"YES\nYES\nNO\nNO\nNO\nYES\nYES\nNO\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADve01Posledovatelnosti"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    input.usize();
    input.usize();
    let s = input.string();
    let t = input.string();
    let pref = s.len() - t.len() + 1;
    let mut has0 = false;
    let mut has1 = false;
    for i in 0..pref {
        if s[i] == b'0' {
            has0 = true;
        } else {
            has1 = true;
        }
    }
    if (t[0] == b'0' && !has0) || (t[0] == b'1' && !has1) || t[1..] != s[pref..] {
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
    // tester::run_stress(stress);
}
//END MAIN

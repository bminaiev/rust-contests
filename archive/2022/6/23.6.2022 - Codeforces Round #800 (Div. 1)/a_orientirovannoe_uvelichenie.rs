//{"name":"A. Ориентированное увеличение","group":"Codeforces - Codeforces Round #800 (Div. 1)","url":"https://codeforces.com/contest/1693/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n2\n1 0\n4\n2 -1 -1 0\n4\n1 -4 3 0\n4\n1 -1 1 -1\n5\n1 2 3 4 -10\n7\n2 -1 1 -2 0 0 0\n1\n0\n","output":"No\nYes\nNo\nNo\nYes\nYes\nYes\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AOrientirovannoeUvelichenie"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut ok = true;
    let mut pref_sum = 0;
    for (pos, &x) in a.iter().enumerate() {
        if pos != 0 && x != 0 && pref_sum == 0 {
            ok = false;
        }
        pref_sum += x;
        if pref_sum < 0 {
            ok = false;
        }
    }
    if pref_sum != 0 {
        ok = false;
    }
    if ok {
        out_line!("Yes");
    } else {
        out_line!("No");
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

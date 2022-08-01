//{"name":"C. Вир&ус","group":"Codeforces - CodeTON Round 2 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1704/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n10 3\n3 6 8\n6 2\n2 5\n20 3\n3 7 12\n41 5\n1 11 21 31 41\n10 5\n2 4 6 8 10\n5 5\n3 2 5 4 1\n1000000000 1\n1\n1000000000 4\n1 1000000000 10 16\n","output":"7\n5\n11\n28\n9\n5\n2\n15\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVirus"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i64();
    let m = input.usize();
    let mut a = input.vec::<i64>(m);
    a.sort();
    a.push(a[0] + n);
    let mut diffs = vec![];
    for i in 0..a.len() - 1 {
        diffs.push(a[i + 1] - a[i] - 1);
    }
    diffs.sort();
    let mut already_removed = 0;
    let mut alive = 0;
    for &d in diffs.iter().rev() {
        let cur = d - already_removed;
        if cur > 0 {
            let cur = cur - 2;
            alive += 1;
            if cur > 0 {
                alive += cur;
            }
        } else {
            break;
        }
        already_removed += 4;
    }
    out_line!(n - alive);
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

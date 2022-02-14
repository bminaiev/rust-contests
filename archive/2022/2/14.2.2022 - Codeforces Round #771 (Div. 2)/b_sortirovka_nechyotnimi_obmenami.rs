//{"name":"B. Сортировка нечётными обменами","group":"Codeforces - Codeforces Round #771 (Div. 2)","url":"https://codeforces.com/contest/1638/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n1 6 31 14\n2\n4 2\n5\n2 9 6 7 10\n3\n6 6 6\n","output":"Yes\nNo\nNo\nYes\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSortirovkaNechyotnimiObmenami"}}}

use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut even = vec![];
    let mut odd = vec![];
    for _ in 0..n {
        let x = input.i32();
        if x % 2 == 0 {
            even.push(x);
        } else {
            odd.push(x);
        }
    }
    if even == even.sorted() && odd == odd.sorted() {
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

//{"name":"B. Расшифровка повторяшек","group":"Codeforces - Codeforces Round #773 (Div. 1)","url":"https://codeforces.com/contest/1641/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n5 7\n2\n5 5\n6\n1 3 1 2 2 3\n6\n3 2 1 1 2 3\n","output":"-1\n0\n1\n2\n4\n1 3\n5 3\n5 3\n10 3\n2\n8 6\n5\n0 3\n8 3\n5 3\n6 2\n7 1\n4\n2 6 6 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRasshifrovkaPovtoryashek"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i32>(n);
    let mut changes = vec![];
    let mut tandems = vec![];
    let mut from = 0;
    while from != a.len() {
        // dbg!(a, from);
        let mut pos_same = from + 1;
        while pos_same != a.len() && a[pos_same] != a[from] {
            pos_same += 1;
        }
        if pos_same == a.len() {
            out_line!(-1);
            return;
        }
        let mut pos_to_add = pos_same + 1;
        for copy in from + 1..pos_same {
            changes.push((pos_to_add, a[copy]));
            a.insert(pos_to_add, a[copy]);
            a.insert(pos_to_add, a[copy]);
            pos_to_add += 1;
        }
        tandems.push((from, pos_to_add));
        from = pos_to_add;
    }
    out_line!(changes.len());
    for &(pos, val) in changes.iter() {
        out_line!(pos, val);
    }
    out_line!(tandems.len());
    for &(fr, to) in tandems.iter() {
        out_line!(to - fr);
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
    // tester::run_single_test("2");
}
//END MAIN

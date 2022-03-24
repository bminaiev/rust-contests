//{"name":"H. Handling the Blocks","group":"Codeforces - 2021-2022 ACM-ICPC Brazil Subregional Programming Contest","url":"https://codeforces.com/gym/103388/problem/H","interactive":false,"timeLimit":500,"tests":[{"input":"4 2\n3 1\n4 2\n1 1\n2 2\n","output":"Y\n"},{"input":"4 2\n2 1\n4 2\n1 1\n3 2\n","output":"N\n"},{"input":"3 1\n1 1\n2 1\n3 1\n","output":"Y\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HHandlingTheBlocks"}}}

use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let colors = input.usize();
    let mut by_color = vec![vec![]; colors];
    let mut a = vec![];
    for pos in 0..n {
        a.push(input.i32());
        by_color[input.usize() - 1].push(pos);
    }
    for c in 0..colors {
        let mut cur = vec![];
        for &pos in by_color[c].iter() {
            cur.push(a[pos]);
        }
        cur.sort();
        for idx in 0..cur.len() {
            a[by_color[c][idx]] = cur[idx];
        }
    }
    if a.sorted() == a {
        out_line!("Y");
    } else {
        out_line!("N");
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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

//{"name":"C. Разноцветная таблица","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2 1\n1 1\n2 2\n1 2\n3 5\n3 2 4\n4 2\n1 2 1 2\n5 3\n1 2 3 2 1\n","output":"4\n4 2\n0 6 6 2 0\n8 6\n10 6 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRaznotsvetnayaTablitsa"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut positions = vec![vec![]; k];
    for i in 0..n {
        positions[a[i]].push(i);
    }
    let mut min_pos = n;
    let mut max_pos = 0;
    let mut res = vec![];
    for i in (0..k).rev() {
        if positions[i].is_empty() {
            res.push(0);
        } else {
            for x in positions[i].iter() {
                min_pos = min_pos.min(*x);
                max_pos = max_pos.max(*x);
            }
            res.push((max_pos - min_pos + 1) * 2);
        }
    }
    res.reverse();
    out_line!(res);
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

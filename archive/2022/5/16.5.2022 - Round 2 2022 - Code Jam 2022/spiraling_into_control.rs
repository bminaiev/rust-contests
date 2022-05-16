//{"name":"Spiraling Into Control","group":"Google Coding Competitions - Round 2 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008778ec/0000000000b15a74","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n5 4\n5 3\n5 12\n3 1\n","output":"Case #1: 2\n2 17\n18 25\nCase #2: IMPOSSIBLE\nCase #3: 2\n11 22\n22 25\nCase #4: IMPOSSIBLE\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SpiralingIntoControl"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_n(n: usize, mut more_skip: usize) -> Option<Vec<(usize, usize)>> {
    let mut cur_value = 1;
    let mut can_skip = 4 * n - 6;
    let mut res = vec![];
    let mut next_lvl = n;
    let mut cur_side = 0;
    while cur_value != n * n && can_skip != 0 {
        if can_skip <= more_skip {
            more_skip -= can_skip;
            res.push((cur_value, cur_value + can_skip + 1));
            cur_value += can_skip + 2;
            if can_skip > 8 {
                can_skip -= 8;
            } else {
                break;
            }
            next_lvl -= 2;
        } else {
            can_skip -= 2;
            cur_value += next_lvl;
            if cur_side == 0 {
                next_lvl -= 1;
            }
            cur_side = (cur_side + 1) % 2;
        }
    }
    if more_skip == 0 {
        Some(res)
    } else {
        None
    }
}

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let need_moves = input.usize();
    let need_skip = (n * n - 1) - need_moves;
    out!(format!("Case #{}: ", test_case));
    if let Some(res) = solve_n(n, need_skip) {
        out_line!(res.len());
        for (x, y) in res.into_iter() {
            out_line!(x, y);
        }
    } else {
        out_line!("IMPOSSIBLE");
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

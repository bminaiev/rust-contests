//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cell {
    color: i32,
    x: i64,
    y: i64,
}

fn solve_same_c(mut all: Vec<i64>) -> i64 {
    all.sort();
    let mut res = 0;
    let mut sum_x = 0;
    let mut cnt = 0;
    for i in 0..all.len() {
        res += all[i] * cnt - sum_x;
        cnt += 1;
        sum_x += all[i];
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut cells = vec![];
    for x in 0..n {
        for y in 0..m {
            let color = input.i32();
            cells.push(Cell {
                color,
                x: x as i64,
                y: y as i64,
            });
        }
    }
    cells.sort();
    let mut res = 0;
    let mut i = 0;
    while i != cells.len() {
        let mut j = i;
        while j != cells.len() && cells[i].color == cells[j].color {
            j += 1;
        }
        let mut all_x = vec![];
        let mut all_y = vec![];
        for k in i..j {
            all_x.push(cells[k].x);
            all_y.push(cells[k].y);
        }
        res += solve_same_c(all_x);
        res += solve_same_c(all_y);
        i = j;
    }
    out_line!(res * 2);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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

//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let mut res = vec![];
    let shifts = [
        Point { x: 0, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: 0 },
    ];
    let dirs = [b'U', b'R', b'D', b'L'];
    for mask_bad in 0..(1 << n) {
        let mut cur = Point { x: 0, y: 0 };
        let mut ok = vec![cur];
        let mut bad = vec![];
        for i in 0..n {
            let mut dir = 4;
            for j in 0..4 {
                if dirs[j] == s[i] {
                    dir = j;
                    break;
                }
            }
            assert_ne!(dir, 4);
            let next = Point {
                x: cur.x + shifts[dir].x,
                y: cur.y + shifts[dir].y,
            };
            if (1 << i) & mask_bad != 0 {
                bad.push(next);
            } else {
                ok.push(next);
                cur = next;
            }
        }
        let mut oks = true;
        for p1 in ok.iter() {
            for p2 in bad.iter() {
                if p1 == p2 {
                    oks = false;
                }
            }
        }
        if oks {
            res.push(cur);
        }
    }
    res.sort();
    res.dedup();
    out_line!(res.len());
    for p in res.iter() {
        out_line!(p.x, p.y);
    }
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

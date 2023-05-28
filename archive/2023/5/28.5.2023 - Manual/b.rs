//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use std::collections::BTreeMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Copy, Clone, Default)]
struct Pos {
    x: usize,
    y: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut pos = vec![Pos::default(); n * m];
    for i in 0..n {
        for j in 0..m {
            let id = input.usize();
            pos[id] = Pos { x: i, y: j };
        }
    }
    let mut set: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut res = n * m;
    for id in 0..n * m {
        let pos = pos[id];
        let mut ok = true;
        if let Some((x, (y1, y2))) = set.range(..pos.x).next_back() {
            if *y2 > pos.y {
                ok = false;
            }
        }

        if let Some((x, (y1, y2))) = set.range(pos.x + 1..).next() {
            if *y1 < pos.y {
                ok = false;
            }
        }
        if !ok {
            res = id;
            break;
        }
        if let Some(&(y1, y2)) = set.get(&pos.x) {
            set.insert(pos.x, (y1.min(pos.y), y2.max(pos.y)));
        } else {
            set.insert(pos.x, (pos.y, pos.y));
        }
    }
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

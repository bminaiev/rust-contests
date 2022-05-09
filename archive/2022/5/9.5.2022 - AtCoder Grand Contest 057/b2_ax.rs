//{"name":"B - 2A + x","group":"AtCoder - AtCoder Grand Contest 057","url":"https://atcoder.jp/contests/agc057/tasks/agc057_b","interactive":false,"timeLimit":4000,"tests":[{"input":"4 2\n5 8 12 20\n","output":"6\n"},{"input":"4 5\n24 25 26 27\n","output":"0\n"},{"input":"4 1\n24 25 26 27\n","output":"3\n"},{"input":"10 5\n39 23 3 7 16 19 40 16 33 6\n","output":"13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B2AX"}}}

use std::cmp::{max, min};
use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Segment {
    r: i128,
    l: i128,
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let x = input.i128();
    let mut seg = BTreeSet::new();
    let mut max_li = 0;
    for _ in 0..n {
        let val = input.i128();
        seg.insert(Segment { l: val, r: val });
        max_li.update_max(val);
    }
    let mut res = std::i128::MAX;
    loop {
        let first = seg.iter().next().unwrap().clone();
        let cur_ans = max_li - first.r;
        res.update_min(cur_ans);
        seg.remove(&first);
        let next = Segment {
            l: first.l * 2,
            r: min(1e20 as i128, first.r * 2 + x),
        };
        if next.l > 1e19 as i128 {
            break;
        }
        seg.insert(next);
        max_li.update_max(next.l);
    }
    out_line!(max(0, res));
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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

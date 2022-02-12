//{"name":"E. Робот на доске 1","group":"Codeforces - Codeforces Round #753 (Div. 3)","url":"https://codeforces.com/contest/1607/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 1\nL\n1 2\nL\n3 3\nRRDLUU\n4 3\nLUURRDDLLLUU\n","output":"1 1\n1 2\n2 1\n3 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERobotNaDoske1"}}}

use algo_lib::geometry::bounding_box::BoundingBox;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::shift_by_uldr;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Pos = PointT<i32>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i32();
    let m = input.i32();
    let s = input.string();
    let mut cur = Pos::ZERO;
    let mut bounding_box = BoundingBox::new(&cur, &cur);
    for &dir in s.iter() {
        let shift = shift_by_uldr(dir);
        cur = cur.apply_shift(&shift);
        let prev_box = bounding_box.clone();
        bounding_box.add(&cur);
        if bounding_box.dx() + 1 > n || bounding_box.dy() + 1 > m {
            bounding_box = prev_box;
            break;
        }
    }
    let start_row = -bounding_box.min.x + 1;
    let start_col = -bounding_box.min.y + 1;
    out_line!(start_row, start_col);
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

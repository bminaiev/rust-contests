//{"name":"A. Кратчайший путь с препятствием","group":"Codeforces - Codeforces Round #731 (Div. 3)","url":"https://codeforces.com/contest/1547/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n\n1 1\n3 3\n2 2\n\n2 5\n2 1\n2 3\n\n1000 42\n1000 1\n1000 1000\n\n1 10\n3 10\n2 10\n\n3 8\n7 8\n3 7\n\n2 1\n4 1\n1 1\n\n1 344\n1 10\n1 1\n","output":"4\n6\n41\n4\n4\n2\n334\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AKratchaishiiPutSPrepyatstviem"}}}

use algo_lib::geometry::bounding_box::BoundingBox;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

fn solve(input: &mut Input, _test_case: usize) {
    let fr: Point = input.read();
    let to: Point = input.read();
    let bad: Point = input.read();
    let bbox = BoundingBox::new(&fr, &to);
    let short = (fr.x - to.x).abs() + (fr.y - to.y).abs();
    let res = if (bbox.dx() != 0 && bbox.dy() != 0) || !bbox.contains(&bad) {
        short
    } else {
        short + 2
    };
    out_line!(res);
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

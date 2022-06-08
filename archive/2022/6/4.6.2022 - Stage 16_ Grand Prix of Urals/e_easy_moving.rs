//{"name":"E. Easy Moving","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/E/","interactive":false,"timeLimit":1000,"tests":[{"input":"NEE\n4\n1 S\n1 W\n2 E\n3 N\n","output":"2 1\n2 -1\n1 0\n1 0\n0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEasyMoving"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::shift_by_nswe;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

fn solve(input: &mut Input, _test_case: usize) {
    let mut s = input.string();
    let q = input.usize();

    let mut cur = Point::ZERO;
    for &c in s.iter() {
        let s = shift_by_nswe(c);
        cur = cur.apply_shift(&s);
    }
    out_line!(cur.y, -cur.x);

    for _ in 0..q {
        let pos = input.usize() - 1;
        let c = input.string()[0];
        let new_s = shift_by_nswe(c);
        cur = cur.apply_shift(&shift_by_nswe(s[pos]).rev());
        s[pos] = c;
        cur = cur.apply_shift(&new_s);
        out_line!(cur.y, -cur.x);
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

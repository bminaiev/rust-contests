//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Person {
    time: i64,
    a: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = gen_vec(n, |_| Person {
        time: input.read(),
        a: input.read(),
    });
    a.sort_by(|p1, p2| (p2.a * (2 * p1.time + 1)).cmp(&(p1.a * (2 * p2.time + 1))));
    let mut cnt_seen = 0;
    let mut res = 0;
    let mut cur_time = 0;
    for p in a.iter() {
        res += p.a * (cur_time + p.time + cnt_seen);
        cur_time += p.time * 2;
        cnt_seen += 1;
    }
    out_line!(res);
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

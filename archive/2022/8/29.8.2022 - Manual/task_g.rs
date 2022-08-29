//{"name":"task_g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"task_g"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let mut state = 0;
    if s[0] == b'r' {
        state = input.u64();
    }
    let mut rnd = Random::new(787788);
    let hashes = gen_vec(1_000_001, |_| rnd.gen_u64());
    let mut map = HashMap::new();
    for i in 0..hashes.len() {
        map.insert(hashes[i], i);
    }
    let cnt = input.usize();
    for _ in 0..cnt {
        let id = input.usize();
        state ^= hashes[id];
        if state == 0 {
            out_line!(0);
        } else if let Some(&id) = map.get(&state) {
            out_line!(id);
        } else {
            out_line!(-1);
        }
    }
    if s[0] == b's' {
        out_line!(state);
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
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // crate::tester::run_locally();
}
//END MAIN

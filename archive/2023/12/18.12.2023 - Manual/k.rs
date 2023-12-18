//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"k"}}}

use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Seq {
    max_suf: i32,
    id: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let a = input.vec::<i32>(n);
    let mut set = BTreeSet::new();
    for id in 1..=k {
        set.insert(Seq { max_suf: 0, id });
    }
    let mut res = vec![];
    for &x in a.iter() {
        let mut use_set = if x == 1 {
            let use_set = set.iter().next().unwrap().clone();
            set.remove(&use_set);
            use_set
        } else {
            assert_eq!(x, -1);
            let use_set = set.iter().next_back().unwrap().clone();
            set.remove(&use_set);
            use_set
        };
        res.push(use_set.id);
        use_set.max_suf += x;
        if use_set.max_suf < 0 {
            use_set.max_suf = 0;
        }
        set.insert(use_set);
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

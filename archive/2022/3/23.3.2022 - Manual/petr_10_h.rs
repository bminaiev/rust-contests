//{"name":"petr_10_h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"file","fileName":"students.in","pattern":null},"output":{"type":"file","fileName":"students.out","pattern":null},"languages":{"java":{"taskClass":"petr_10_h"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    level: i32,
    want_strong: bool,
}

fn solve(input: &mut Input, _test_case: usize) {
    let group_size = input.usize() * 2;
    let num_groups = input.usize();
    let n = group_size * num_groups;
    let levels = input.vec::<i32>(n);
    let want = input.vec::<i32>(n);
    let mut a = gen_vec(n, |id| Person {
        level: levels[id],
        want_strong: want[id] == 1,
    });
    a.sort();
    let mut groups = vec![vec![]; num_groups];
    let mut strong_iter = 0;
    let mut weak_iter = 0;
    let mut ok = 0i32;
    for p in a.iter() {
        while groups[strong_iter].len() == group_size {
            strong_iter += 1;
        }
        if p.want_strong {
            if groups[strong_iter].len() >= group_size / 2 {
                ok += 1;
            }
            groups[strong_iter].push(p.level);
        } else {
            while weak_iter != groups.len() && groups[weak_iter].len() >= group_size / 2 {
                weak_iter += 1;
            }
            if weak_iter == groups.len() {
                groups[strong_iter].push(p.level);
            } else {
                ok += 1;
                groups[weak_iter].push(p.level);
            }
        }
    }
    out_line!(ok);
    for group in groups.into_iter() {
        out_line!(group);
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
        input: TaskIoType::File("students.in".to_string()),
        output: TaskIoType::File("students.out".to_string()),
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

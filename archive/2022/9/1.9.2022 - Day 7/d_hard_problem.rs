//{"name":"D. Hard Problem","group":"Yandex - Day 7","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39552/problems/D/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6 0\n3 1 3 1 3 1\n8 4\n5 8 4 6 5 7 8 5\n7 3\n2 1 3 2 2 1 3\n","output":"144768\n745933\n448953\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DHardProblem"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {}

type Mod = Mod_998_244_353;

fn stress() {
    let mut f = vec![
        Mod::new(3240),
        Mod::new(3081),
        Mod::new(2841),
        Mod::new(343),
    ];
    for i in 4..1000 {
        let sz = f.len();
        let fi1 = f[sz - 1];
        let fi2 = f[sz - 2];
        let fi3 = f[sz - 3];
        let fi4 = f[sz - 4];
        f.push(
            fi1 * Mod::new(223) + fi2 * Mod::new(229) + fi3 * fi4 * Mod::new(239) + Mod::new(17),
        );
        out_line!(f[sz]);
    }
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
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_stress(stress);
}
//END MAIN

//{"name":"H. Beacon Towers","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/H/","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 4 2 5 3\n","output":"6\n"},{"input":"3\n3 2 1\n","output":"1\n"},{"input":"8\n6 3 1 7 2 5 4 8\n","output":"20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HBeaconTowers"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);

    let mut cur_max = 0;

    let mut prev_dp = Mod::ZERO;

    let mut sum = Mod::ZERO;
    for i in 0..n {
        cur_max.update_max(a[i]);
        if cur_max == a[i] {
            prev_dp = sum + Mod::ONE;
        }
        sum += prev_dp;
    }

    out_line!(prev_dp);
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
    // tester::run_single_test("4");
    // tester::run_stress(stress);
}
//END MAIN

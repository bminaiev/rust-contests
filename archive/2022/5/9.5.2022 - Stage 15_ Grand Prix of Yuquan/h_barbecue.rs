//{"name":"H. Barbecue","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/H/","interactive":false,"timeLimit":1500,"tests":[{"input":"7 3\npotatop\n1 3\n3 5\n1 6\n","output":"Putata\nBudada\nBudada\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HBarbecue"}}}

use algo_lib::collections::reversed::ReversedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod9;
use algo_lib::strings::hash_string_context::HashContext;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let s = input.string();

    let context = HashContext::new(n + 1, Mod9::new(239));
    let s1 = context.make_string(&s);
    let s2 = context.make_string(&s.reversed());

    for _ in 0..q {
        let fr = input.usize() - 1;
        let to = input.usize();

        let h1 = s1.calc_hash(fr..to);
        let h2 = s2.calc_hash(n - to..n - fr);

        let len = to - fr;

        if h1 == h2 || (len % 2 == 0) {
            out_line!("Budada");
        } else {
            out_line!("Putata");
        }
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

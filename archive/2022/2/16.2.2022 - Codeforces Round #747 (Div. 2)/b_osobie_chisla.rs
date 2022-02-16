//{"name":"B. Особые числа","group":"Codeforces - Codeforces Round #747 (Div. 2)","url":"https://codeforces.com/contest/1594/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 4\n2 12\n105 564\n","output":"9\n12\n3595374\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOsobieChisla"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

fn solve(input: &mut Input, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.i32();
        let mut k = input.usize();
        let mut res = Mod::ZERO;
        let mut cur = Mod::ONE;
        while k != 0 {
            if (k & 1) == 1 {
                res += cur;
            }
            k /= 2;
            cur *= Mod::new(n);
        }
        out_line!(res);
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

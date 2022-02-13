//{"name":"A - Floor, Ceil - Decomposition","group":"AtCoder - AtCoder Regular Contest 135","url":"https://atcoder.jp/contests/arc135/tasks/arc135_a","interactive":false,"timeLimit":2000,"tests":[{"input":"15\n","output":"192\n"},{"input":"3\n","output":"3\n"},{"input":"100\n","output":"824552442\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AFloorCeilDecomposition"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input) {
    let mut cache: HashMap<i64, Mod> = HashMap::new();
    let ask = input.i64();
    let ans = RecursiveFunction::new(|f, x: i64| -> Mod {
        if let Some(&res) = cache.get(&x) {
            return res;
        }
        if x <= 3 {
            return Mod::new(x as i32);
        }
        let f1 = x / 2;
        let f2 = (x + 1) / 2;
        let res = f.call(f1) * f.call(f2);
        cache.insert(x, res);
        res
    })
    .call(ask);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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

//{"name":"Ex - Dice Product 2","group":"AtCoder - Denso Create Programming Contest 2022(AtCoder Beginner Contest 239)","url":"https://atcoder.jp/contests/abc239/tasks/abc239_h","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1\n","output":"2\n"},{"input":"2 39\n","output":"12\n"},{"input":"3 2\n","output":"250000004\n"},{"input":"2392 39239\n","output":"984914531\n"},{"input":"1000000000 1000000000\n","output":"776759630\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ExDiceProduct2"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

fn solve(input: &mut Input) {
    let n = input.i32();
    let need = input.usize() + 1;

    let mut cache = vec![None; 10_000_000];

    let res = RecursiveFunction::new(|f, need: usize| {
        if need == 1 {
            return Mod::ZERO;
        }
        if need < cache.len() {
            if let Some(res) = cache[need] {
                return res;
            }
        }
        let mut res = Mod::ZERO;
        let mut div = 2;
        while div != (n + 1) as usize {
            let next = (need + div - 1) / div;
            assert!(next > 0);
            let ndiv = min(
                n as usize + 1,
                if next == 1 {
                    std::usize::MAX
                } else {
                    (need + next - 2) / (next - 1)
                },
            );
            res += f.call(next) * Mod::new((ndiv - div) as i32);
            div = ndiv;
        }
        res /= Mod::new(n - 1);
        res += Mod::ONE;
        if need < cache.len() {
            cache[need] = Some(res);
        }
        res
    })
    .call(need);

    let pr_ok = Mod::new(n - 1) / Mod::new(n);
    let res = res / pr_ok;
    out_line!(res);
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

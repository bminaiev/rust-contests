//{"name":"E. Почти идеально","group":"Codeforces - Codeforces Round #819 (Div. 1 + Div. 2) and Grimoire of Code Annual Contest 2022","url":"https://codeforces.com/contest/1726/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n2\n3\n50\n","output":"2\n4\n830690567\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPochtiIdealno"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut res = Mod::ZERO;
    let cnk = CombinationsFact::<Mod>::new(n);
    let powers = Mod::gen_powers(Mod::TWO, n + 1);
    for cnt1 in 0..=n {
        for cnt2 in 0..=n {
            for cnt4 in 0..=n {
                if cnt1 + cnt2 * 2 + cnt4 * 4 == n {
                    let tot = cnt1 + cnt2 * 2 + cnt4 * 2;
                    let ways = cnk.c(tot, cnt1)
                        * cnk.c(tot - cnt1, cnt2 * 2)
                        * cnk.c(cnt2 * 2, cnt2)
                        * cnk.c(cnt4 * 2, cnt4)
                        * cnk.fact(cnt2)
                        * cnk.fact(cnt4)
                        / powers[cnt2]
                        / powers[cnt4];
                    res += ways;
                }
            }
        }
    }
    out_line!(res);
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN

//{"name":"B. Марин и не взаимно простая перестановка","group":"Codeforces - Codeforces Round #779 (Div. 2)","url":"https://codeforces.com/contest/1658/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n1\n2\n3\n4\n5\n6\n1000\n","output":"0\n1\n0\n4\n0\n36\n665702330\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMarinINeVzaimnoProstayaPerestanovka"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::factorials::gen_facts;
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    type Mod = Mod_998_244_353;

    let n = input.usize();
    let facts = gen_facts::<Mod>(n + 1);
    let res = if n % 2 == 1 {
        Mod::ZERO
    } else {
        facts[n / 2] * facts[n / 2]
    };
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
}
//END MAIN

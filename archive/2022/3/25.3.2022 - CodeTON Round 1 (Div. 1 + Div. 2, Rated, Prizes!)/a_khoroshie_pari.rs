//{"name":"A. Хорошие пары","group":"Codeforces - CodeTON Round 1 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1656/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n5 2 7\n5\n1 4 2 2 3\n1\n2\n","output":"2 3\n1 2\n1 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AKhoroshiePari"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Elem {
    pos: usize,
    val: i32,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = gen_vec(n, |pos| Elem {
        pos,
        val: input.read(),
    });
    a.sort_by_key(|e| e.val);
    out_line!(a[0].pos + 1, a[n - 1].pos + 1);
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

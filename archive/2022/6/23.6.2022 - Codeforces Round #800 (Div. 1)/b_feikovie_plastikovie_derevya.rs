//{"name":"B. Фейковые пластиковые деревья","group":"Codeforces - Codeforces Round #800 (Div. 1)","url":"https://codeforces.com/contest/1693/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1\n1 5\n2 9\n3\n1 1\n4 5\n2 4\n6 10\n4\n1 2 1\n6 9\n5 6\n4 5\n2 4\n5\n1 2 3 4\n5 5\n4 4\n3 3\n2 2\n1 1\n","output":"1\n2\n2\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFeikoviePlastikovieDerevya"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut p = vec![0; n];
        for i in 1..n {
            p[i] = input.usize() - 1;
        }
        let mut l = vec![0; n];
        let mut r = vec![0; n];
        for i in 0..n {
            l[i] = input.i64();
            r[i] = input.i64();
        }
        let mut already = vec![0; n];
        let mut res = 0;
        for v in (0..n).rev() {
            if already[v] >= l[v] {
                already[v].update_min(r[v]);
            } else {
                already[v] = r[v];
                res += 1;
            }
            already[p[v]] += already[v];
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
    // tester::run_stress(stress);
}
//END MAIN

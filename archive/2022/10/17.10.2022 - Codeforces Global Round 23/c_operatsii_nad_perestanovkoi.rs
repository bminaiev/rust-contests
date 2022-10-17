//{"name":"C. Операции над перестановкой","group":"Codeforces - Codeforces Global Round 23","url":"https://codeforces.com/contest/1746/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n1 2 3 4\n5\n1 3 2 4 5\n3\n2 3 1\n1\n1\n","output":"1 1 1 1\n1 4 3 2 1\n1 3 3\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COperatsiiNadPerestanovkoi"}}}

use algo_lib::collections::permutation::Permutation;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let p = Permutation::from_vec(input.vec(n).sub_from_all(1));
    for i in (0..n) {
        let pos = (p.get_pos_of_element(i) + 1) % n;
        out!(pos + 1, "");
    }
    out_line!();
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
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN

//{"name":"E. Qwerty78 Trip","group":"Codeforces - 2015 AlBaath Collegiate Programming Contest","url":"https://codeforces.com/gym/100947/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n2 3\n1 2\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EQwerty78Trip"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

struct Test {
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let tc = input.usize();
    let tests = gen_vec(tc, |_| Test {
        rows: input.usize(),
        cols: input.usize(),
        r: input.usize() - 1,
        c: input.usize() - 1,
    });
    let max_sum = tests.iter().map(|t| t.rows + t.cols).max().unwrap();
    let cnk = CombinationsFact::<Mod>::new(max_sum + 2);
    for t in tests {
        let total_ways = cnk.c(t.rows + t.cols - 2, t.rows - 1);
        let bad_ways =
            cnk.c(t.r + t.c, t.r) * cnk.c(t.rows - t.r - 1 + t.cols - t.c - 1, t.rows - t.r - 1);
        let ans = total_ways - bad_ways;
        out_line!(ans);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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

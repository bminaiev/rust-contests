//{"name":"B. Фитнес Элы и роскошные числа","group":"Codeforces - Dytechlab Cup 2022","url":"https://codeforces.com/contest/1737/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n8 19\n8 20\n119 121\n1 100000000000000000\n1234567891011 1000000000000000000\n","output":"5\n6\n2\n948683296\n2996666667\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFitnesEliIRoskoshnieChisla"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_last_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_r(mx: i64) -> i64 {
    if mx == 3 {
        return 3;
    }
    let mut res = 0;
    if 3 <= mx {
        res += 1;
    }
    if 4 <= mx {
        res -= 1;
    }
    let max_check = ((mx as f64).sqrt() as i64) + 10;
    res += binary_search_last_true(0..max_check, |c| c * c <= mx).unwrap();
    res += binary_search_last_true(0..max_check, |c| c * (c + 1) <= mx).unwrap();
    res += binary_search_last_true(0..max_check, |c| c * (c + 2) <= mx).unwrap();
    res
}

fn stress() {
    let mut res = 0;
    for x in 1..=1000 {
        let c = (x as f64).sqrt() as i64;
        if x % c == 0 {
            res += 1;
        }
        let my = solve_r(x);
        if my != res {
            dbg!(x, my, res);
        }
        // assert_eq!(my, res);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let l = input.i64();
    let r = input.i64();
    out_line!(solve_r(r) - solve_r(l - 1));
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

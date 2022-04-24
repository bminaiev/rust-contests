//{"name":"E. блокнот.exe","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/E","interactive":true,"timeLimit":1000,"tests":[{"input":"6\n? 1\n\n? 9\n\n? 16\n\n! 32\n","output":"\n0\n\n4\n\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBloknotexe"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn query(input: &mut Input, w: i64, max_queries: &mut i64) -> i64 {
    if *max_queries == 0 {
        loop {}
    }
    *max_queries -= 1;
    out_line!("?", w);
    output().flush();
    input.i64()
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i64();
    let max_sum_len = n * 2001 + 5;
    let mut max_queries = n + 30;
    let one_line = binary_search_first_true(1..max_sum_len, |len| {
        query(input, len, &mut max_queries) == 1
    });
    let mut res = one_line;
    let smallest_possible_ans = res - (n - 1);
    let mut rnd = Random::new(787788);
    for &lines in rnd.gen_permutation(n as usize + 1).iter() {
        let lines = lines as i64;
        if lines <= 1 {
            continue;
        }
        loop {
            let w = (res - 1) / lines;
            if w * lines < smallest_possible_ans {
                break;
            }
            let r = query(input, w, &mut max_queries);
            if r != 0 && r <= lines {
                res = r * w;
            } else {
                break;
            }
        }
    }

    out_line!("!", res);
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
        is_interactive: true,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_locally();
}
//END MAIN

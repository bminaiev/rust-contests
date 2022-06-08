//{"name":"F. Движущаяся строка","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n5\nababa\n3 4 5 2 1\n5\nababa\n2 1 4 5 3\n10\ncodeforces\n8 6 1 7 5 2 9 3 10 4\n","output":"1\n6\n12\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FDvizhushchayasyaStroka"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::lcm;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let mut seen = vec![false; n];
    let p = input.vec::<usize>(n).sub_from_all(1);
    let mut res = 1;
    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut cycle = vec![];
        let mut cur = i;
        while !seen[cur] {
            cycle.push(cur);
            seen[cur] = true;
            cur = p[cur];
        }
        for smallest_shift in 1..=cycle.len() {
            let mut ok = true;
            for i in 0..cycle.len() {
                let mut ni = i + smallest_shift;
                if ni >= cycle.len() {
                    ni -= cycle.len();
                }
                if s[cycle[i]] != s[cycle[ni]] {
                    ok = false;
                    break;
                }
            }
            if ok {
                res = lcm(res, smallest_shift as i64);
                break;
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
}
//END MAIN

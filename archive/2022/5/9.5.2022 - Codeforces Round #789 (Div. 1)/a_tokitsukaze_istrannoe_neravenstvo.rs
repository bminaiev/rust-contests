//{"name":"A. Tokitsukaze и странное неравенство","group":"Codeforces - Codeforces Round #789 (Div. 1)","url":"https://codeforces.com/contest/1677/problem/0","interactive":false,"timeLimit":1500,"tests":[{"input":"3\n6\n5 3 6 1 4 2\n4\n1 2 3 4\n10\n5 1 6 2 8 3 4 10 9 7\n","output":"3\n0\n28\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATokitsukazeIStrannoeNeravenstvo"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let p = input.vec::<usize>(n).sub_from_all(1);
    let mut left = Array2D::new(0, n, n);
    let mut cur_cnt = vec![0; n];
    for i in 0..n {
        for j in i + 1..n {
            left[i][j] = cur_cnt[p[j]];
        }
        for x in p[i]..n {
            cur_cnt[x] += 1;
        }
    }
    let mut right = Array2D::new(0, n, n);
    let mut cur_cnt = vec![0; n];
    for i in (0..n).rev() {
        for j in (0..i).rev() {
            right[j][i] = cur_cnt[p[j]];
        }
        for x in p[i]..n {
            cur_cnt[x] += 1;
        }
    }
    let mut res = 0i64;
    for i in 0..n {
        for j in i + 1..n {
            let a = left[i][j] as i64;
            let b = right[i][j] as i64;
            res += a * b;
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

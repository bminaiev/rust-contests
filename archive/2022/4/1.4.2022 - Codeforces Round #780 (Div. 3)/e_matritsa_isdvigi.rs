//{"name":"E. Матрица и сдвиги","group":"Codeforces - Codeforces Round #780 (Div. 3)","url":"https://codeforces.com/contest/1660/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n\n3\n010\n011\n100\n\n5\n00010\n00001\n10000\n01000\n00100\n\n2\n10\n10\n\n4\n1111\n1011\n1111\n1111\n","output":"1\n0\n2\n11\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMatritsaISdvigi"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = gen_vec(n, |_| input.string());
    let mut a = Array2D::new(0, n * 2 + 1, n * 2 + 1);
    for i in 0..2 * n {
        for j in 0..2 * n {
            a[i + 1][j + 1] = if s[i % n][j % n] == b'1' { 1 } else { 0 };
            a[i + 1][j + 1] += a[i][j];
        }
    }
    let mut tot_ones = 0;
    for i in 0..n {
        tot_ones += a[i + n][n] - a[i][0];
    }
    let mut max_ones = 0;
    for i in 0..n {
        for j in 0..n {
            max_ones.update_max(a[i + n][j + n] - a[i][j]);
        }
    }
    out_line!((tot_ones - max_ones) + (n - max_ones));
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

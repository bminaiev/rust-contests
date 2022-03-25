//{"name":"F. Параметрическое MST","group":"Codeforces - CodeTON Round 1 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1656/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n1 0\n2\n-1 1\n3\n1 -1 -2\n3\n3 -1 -2\n4\n1 2 3 -4\n","output":"INF\n-1\nINF\n-6\n-18\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FParametricheskoeMST"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_a(a: &mut Vec<i64>) -> Option<i64> {
    a.sort();
    let mut res = i64::MIN;
    let n = a.len();
    if n == 2 {
        let constant = a[0] * a[1];
        let mult = a[0] + a[1];
        if mult != 0 {
            return None;
        }
        return Some(constant);
    }
    {
        // check t = +inf
        let mut coef = 0;
        for i in 1..n {
            coef += a[i] + a[0];
        }
        if coef > 0 {
            return None;
        }
    }
    {
        // check t = -inf
        let mut coef = 0;
        for i in 0..n - 1 {
            coef += a[i] + a[n - 1];
        }
        if coef < 0 {
            return None;
        }
    }
    let mut sum_left = 0;
    let mut sum_right: i64 = a[1..].iter().sum();
    for center in 1..n - 1 {
        sum_left += a[center - 1];
        sum_right -= a[center];
        let t = -a[center];
        let cnt_left = center as i64;
        let cnt_right = (n - 1) as i64 - cnt_left;
        let from_right = sum_right * a[0] + t * sum_right + t * a[0] * cnt_right;
        let from_left = sum_left * a[n - 1] + t * sum_left + t * a[n - 1] * cnt_left;
        let from_center = a[center] * t;
        let cur_res =
            from_left + from_right - a[0] * a[n - 1] - t * (a[0] + a[n - 1]) + from_center;
        res.update_max(cur_res);
    }
    Some(res)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i64>(n);
    if let Some(r) = solve_a(&mut a) {
        out_line!(r);
    } else {
        out_line!("INF");
    }
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

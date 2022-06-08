//{"name":"E. Максимизация цен","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6 3\n3 2 7 1 4 8\n4 3\n2 1 5 6\n4 12\n0 0 0 0\n2 1\n1 1\n6 10\n2 0 0 5 9 4\n6 5\n5 3 8 6 3 2\n","output":"8\n4\n0\n2\n1\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMaksimizatsiyaTsen"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.i64();
    let a = input.vec::<i64>(n);
    let mut already = 0;
    let mut left = vec![];
    for &x in a.iter() {
        already += x / k;
        left.push(x % k);
    }
    left.sort();
    let mut l = 0;
    let mut r = n - 1;
    for _ in 0..n / 2 {
        if left[l] + left[r] >= k {
            already += 1;
            l += 1;
            r -= 1;
        } else {
            l += 2;
        }
    }
    out_line!(already);
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

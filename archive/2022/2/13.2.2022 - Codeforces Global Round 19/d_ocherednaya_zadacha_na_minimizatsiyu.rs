//{"name":"D. Очередная задача на минимизацию","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1\n3\n6\n4\n3 6 6 6\n2 7 4 1\n4\n6 7 2 4\n2 5 3 5\n","output":"0\n987\n914\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DOcherednayaZadachaNaMinimizatsiyu"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i64>(n);
    let b = input.read_vec::<i64>(n);
    let mut total_sum = 0;
    let mut dp = vec![0];
    for (&x, &y) in a.iter().zip(b.iter()) {
        let max_add = (x + y) as usize;
        let mut ndp = vec![i64::MAX; dp.len() + max_add];
        for sum_in_a in 0..dp.len() {
            let cur = dp[sum_in_a];
            if cur == i64::MAX {
                continue;
            }
            let always_add = (x * x + y * y) * ((n - 1) as i64);
            let sum_in_b = total_sum - sum_in_a;
            ndp[sum_in_a + x as usize]
                .update_min(cur + always_add + sum_in_a as i64 * x * 2 + sum_in_b as i64 * y * 2);
            ndp[sum_in_a + y as usize]
                .update_min(cur + always_add + sum_in_a as i64 * y * 2 + sum_in_b as i64 * x * 2);
        }
        dp = ndp;
        total_sum += (x + y) as usize;
    }
    let res = *dp.iter().min().unwrap();
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

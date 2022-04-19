//{"name":"B. Оптимальное разбиение","group":"Codeforces - Codeforces Round #783 (Div. 1)","url":"https://codeforces.com/contest/1667/problem/B","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n3\n1 2 -3\n4\n0 -2 3 -4\n5\n-1 -2 3 -1 -1\n6\n-1 2 -3 4 -5 6\n7\n1 -1 -1 1 -1 -1 1\n","output":"1\n2\n1\n6\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOptimalnoeRazbienie"}}}

use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::seg_trees::fenwick_max::FenwickMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let pref = a.pref_sum();
    let all = pref.clone().sorted();
    let idx = |value: i64| -> usize { all.binary_search(&value).unwrap() };
    let mut fenw_max = FenwickMax::<i64>::new(n + 1);
    fenw_max.add(idx(0), 0);
    fenw_max.add(0, std::i64::MIN / 2);
    let mut dp = vec![std::i64::MIN / 2; n + 1];
    dp[idx(0)] = 0;
    let mut global_max = 0;
    for pos in 0..n {
        let sum = pref[pos + 1];
        let idx = idx(sum);
        let mut cur = i64::MIN;
        if idx > 0 {
            cur.update_max(fenw_max.get_range_max(idx - 1) + (pos + 1) as i64);
        }
        cur.update_max(dp[idx]);
        cur.update_max(global_max - (pos + 1) as i64);
        dp[idx].update_max(cur);
        global_max.update_max(cur + (pos + 1) as i64);
        fenw_max.add(idx, cur - (pos + 1) as i64);
        if pos == n - 1 {
            out_line!(cur);
        }
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

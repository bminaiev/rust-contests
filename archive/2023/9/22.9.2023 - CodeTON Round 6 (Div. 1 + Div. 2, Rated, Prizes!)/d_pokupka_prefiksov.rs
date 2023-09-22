//{"name":"D. Покупка префиксов","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 3\n5\n2\n3 4\n7\n3\n3 2 1\n2\n6\n10 6 4 6 3 4\n7\n","output":"5 0 0\n2 1\n2 2 2\n2 2 2 2 2 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPokupkaPrefiksov"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_last_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut c = input.vec::<i64>(n);
    let mut k = input.i64();
    for i in (0..n - 1).rev() {
        c[i] = min(c[i], c[i + 1]);
    }
    let mut res = vec![0; n];
    res[0] = k / c[0];
    for i in 1..n {
        if res[i - 1] * c[i] <= k {
            res[i] = res[i - 1];
        } else {
            let cnt_new = binary_search_last_true(0..res[i - 1], |cnt| {
                cnt * c[i] + (res[i - 1] - cnt) * c[i - 1] <= k
            })
            .unwrap();
            res[i] = cnt_new;
            k -= (res[i - 1] - cnt_new) * c[i - 1];
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

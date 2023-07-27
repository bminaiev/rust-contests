//{"name":"E1. Рудольф и снежинки (простая версия)","group":"Codeforces - Codeforces Round 883 (Div. 3)","url":"https://codeforces.com/contest/1846/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n1\n2\n3\n6\n13\n15\n255\n10101\n1000000\n","output":"NO\nNO\nNO\nNO\nYES\nYES\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1RudolfISnezhinkiProstayaVersiya"}}}

use std::collections::HashSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn find_k(sum: i64) -> bool {
    let k = binary_search_first_true(2..1e9 as i64, |k| 1 + k + k * k >= sum);
    1 + k + k * k == sum
}

fn solve(input: &mut Input, _test_case: usize) {
    const MAX_SZ: i64 = 1e18 as i64 + 100;
    // 1 -> [1, k] -> [1, k, k * k]
    let mut exist = HashSet::new();
    for k in 2..1_000_000i64 {
        let mut total = 1 + k;
        let mut lvl = k;
        loop {
            if MAX_SZ / k < lvl {
                break;
            }
            lvl *= k;
            total += lvl;
            exist.insert(total);
        }
    }
    let tc = input.usize();
    for _ in 0..tc {
        let query = input.i64();
        if exist.contains(&query) || find_k(query) {
            out_line!("YES");
        } else {
            out_line!("NO");
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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

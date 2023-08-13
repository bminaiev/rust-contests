//{"name":"C. Горнолыжный курорт","group":"Codeforces - Codeforces Round 878 (Div. 3)","url":"https://codeforces.com/contest/1840/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n3 1 15\n-5 0 -10\n5 3 -33\n8 12 9 0 5\n4 3 12\n12 12 10 15\n4 1 -5\n0 -1 2 5\n5 5 0\n3 -1 4 -5 -3\n1 1 5\n5\n6 1 3\n0 3 -2 5 -4 -4\n","output":"6\n0\n1\n0\n0\n1\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGornolizhniiKurort"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let q = input.i64();
        let good = gen_vec(n, |_| input.i64() <= q);
        let mut res = 0;
        let mut i = 0;
        while i < n {
            if !good[i] {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            while j != n && good[j] {
                j += 1;
            }
            while i != j {
                let len = j - i;
                if len >= k {
                    res += len - k + 1;
                }
                i += 1;
            }
        }
        out_line!(res);
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

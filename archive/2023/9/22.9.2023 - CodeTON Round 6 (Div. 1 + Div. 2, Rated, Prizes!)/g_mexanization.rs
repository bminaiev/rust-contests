//{"name":"G. MEXanization","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n8\n179 57 2 0 2 3 2 3\n1\n0\n3\n1 0 3\n8\n1 0 1 2 4 3 0 2\n","output":"179 2 3 3 3 4 4 5\n1\n1 2 2\n1 2 2 3 3 5 5 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GMEXanization"}}}

use std::cmp::{max, min};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_case(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut res = vec![0; n];
    let mx_val = n + 3;
    let mut cur_res = 0;
    const BLOCK_LOG: usize = 5;
    const BLOCK_MASK: usize = (1 << BLOCK_LOG) - 1;
    let mut cnt = vec![0i64; mx_val];
    let mut block_min = vec![0i64; (mx_val >> BLOCK_LOG) + 1];
    for i in 0..n {
        let value = if a[i] >= mx_val { 0 } else { a[i] };
        cnt[value] += 1;
        let block_start = value & !BLOCK_MASK;
        let block_end = min(mx_val, block_start + BLOCK_MASK + 1);
        block_min[value >> BLOCK_LOG] = cnt[block_start..block_end].iter().copied().min().unwrap();
        loop {
            let mut need = 1i64;
            let mut total_alive = (i + 1) as i64;
            let check_res = cur_res + 1;
            total_alive -= check_res as i64;
            for block_id in (0..=check_res >> BLOCK_LOG).rev() {
                if total_alive < 0 {
                    break;
                }
                let block_start = block_id << BLOCK_LOG;
                let block_end = block_start + BLOCK_MASK + 1;
                if block_min[block_id] >= need {
                    continue;
                }
                for p in (max(1, block_start)..min(block_end, check_res)).rev() {
                    let new_here = max(0, need - cnt[p]);
                    total_alive -= new_here * (p as i64 - 1);
                    need += new_here;
                    if total_alive < 0 {
                        break;
                    }
                }
            }
            if total_alive >= 0 {
                cur_res = check_res;
            } else {
                break;
            }
        }
        res[i] = cur_res;
    }
    res[0] = max(1, a[0]);
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);
    let res = solve_case(&a);
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
    // tester::run_tests();
    tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN

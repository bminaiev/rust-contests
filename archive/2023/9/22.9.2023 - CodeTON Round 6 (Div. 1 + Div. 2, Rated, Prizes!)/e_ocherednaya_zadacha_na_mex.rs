//{"name":"E. Очередная задача на MEX","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1 0\n10\n1 2 0 7 1 2 0 2 4 3\n10\n2 1 0 7 1 2 0 2 4 3\n3\n1 2 1\n","output":"2\n6\n7\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EOcherednayaZadachaNaMEX"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);
    let mut res_ends_here = vec![vec![]; n + 1];
    res_ends_here[0].push(0);
    let mut last_mex_it = vec![0; n + 1];
    let mut seen_res = vec![false; (n + 1).next_power_of_two() * 2];
    seen_res[0] = true;
    for right in 0..n {
        let mut seen = vec![false; n + 1];
        let mut cur_mex = 0;
        let mut new_res_ends_here = vec![];
        for left in (0..=right).rev() {
            seen[a[left]] = true;
            while seen[cur_mex] {
                cur_mex += 1;
            }
            while last_mex_it[cur_mex] <= left {
                for prev_xor in res_ends_here[last_mex_it[cur_mex]].iter() {
                    let res = prev_xor ^ cur_mex;
                    if !seen_res[res] {
                        seen_res[res] = true;
                        new_res_ends_here.push(res);
                    }
                }
                last_mex_it[cur_mex] += 1;
            }
        }
        res_ends_here[right + 1] = new_res_ends_here;
    }
    let mut res = seen_res.len() - 1;
    while !seen_res[res] {
        res -= 1;
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

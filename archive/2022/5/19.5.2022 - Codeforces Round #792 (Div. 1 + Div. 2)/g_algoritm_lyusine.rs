//{"name":"G. Алгоритм Люсине","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"7 20\n1 8 1 6 3 2 3\n","output":"3\n19 11\n15 9\n3 7\n"},{"input":"2 10\n7 1\n","output":"-1\n"},{"input":"2 15\n1 7\n","output":"1\n15 8\n"},{"input":"1 1000000000\n845063470\n","output":"-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GAlgoritmLyusine"}}}

use algo_lib::flows::matching::find_matching;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.i64();
    let a = input.vec::<i64>(n);

    let mut left = vec![];
    let mut right = vec![];
    for x in a.iter() {
        if 3 * x <= m {
            right.push(x);
        } else {
            left.push(x);
        }
    }

    let mut g = vec![vec![]; left.len()];
    for i in 0..left.len() {
        for j in 0..right.len() {
            if 2 * left[i] + right[j] <= m && left[i] % right[j] == 0 {
                g[i].push(j);
            }
        }
    }

    let matching_res = find_matching(&g, right.len());
    if matching_res.size != left.len() {
        out_line!(-1i32);
        return;
    }

    let mut res = vec![];
    let mut used_right = vec![false; right.len()];
    for i in 0..left.len() {
        let j = matching_res.right[i].unwrap();
        used_right[j] = true;

        res.push((2 * left[i] + right[j], left[i] + right[j]));
    }
    for j in 0..right.len() {
        if !used_right[j] {
            res.push((right[j] * 3, right[j] * 2));
        }
    }
    out_line!(res.len());
    for &(x, y) in res.iter() {
        out_line!(x, y);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = 1; //input.read();
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

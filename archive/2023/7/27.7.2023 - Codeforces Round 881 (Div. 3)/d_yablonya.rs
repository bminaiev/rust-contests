//{"name":"D. Яблоня","group":"Codeforces - Codeforces Round 881 (Div. 3)","url":"https://codeforces.com/contest/1843/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n5\n1 2\n3 4\n5 3\n3 2\n4\n3 4\n5 1\n4 4\n1 3\n3\n1 2\n1 3\n3\n1 1\n2 3\n3 1\n","output":"2\n2\n1\n4\n4\n1\n2\n"},{"input":"2\n5\n5 1\n1 2\n2 3\n4 3\n2\n5 5\n5 1\n5\n3 2\n5 3\n2 1\n4 2\n3\n4 3\n2 1\n4 2\n","output":"1\n2\n1\n4\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYablonya"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut res = vec![0i64; n];
    RecursiveFunction2::new(|f, v: usize, p: usize| {
        for &to in g[v].iter() {
            if to != p {
                res[v] += f.call(to, v);
            }
        }
        if res[v] == 0 {
            res[v] = 1;
        }
        res[v]
    })
    .call(0, 0);
    let qq = input.usize();
    for _q in 0..qq {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        out_line!(res[fr] * res[to]);
    }
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

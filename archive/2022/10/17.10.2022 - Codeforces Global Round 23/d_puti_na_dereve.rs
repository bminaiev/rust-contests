//{"name":"D. Пути на дереве","group":"Codeforces - Codeforces Global Round 23","url":"https://codeforces.com/contest/1746/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n5 4\n1 2 1 3\n6 2 1 5 7\n5 3\n1 2 1 3\n6 6 1 4 10\n","output":"54\n56\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPutiNaDereve"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.i64();
    let mut g = vec![vec![]; n];
    for i in 1..n {
        let prev = input.usize() - 1;
        g[prev].push(i);
    }
    let s = input.vec::<i64>(n);
    let res = RecursiveFunction2::new(|f, v: usize, need: i64| -> [i64; 2] {
        let mut res = [need * s[v], s[v]];
        if g[v].len() != 0 {
            let every = need / (g[v].len() as i64);
            let mut more = vec![];
            for &to in g[v].iter() {
                let child = f.call(to, every);
                res[0] += child[0];
                more.push(child[1]);
            }
            more.sort();
            more.reverse();
            let need_more = (need - every * (g[v].len() as i64)) as usize;
            assert!(need_more < more.len());
            for i in 0..need_more {
                res[0] += more[i];
            }
            res[1] += more[need_more];
        }
        res
    })
    .call(0, k);
    out_line!(res[0]);
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
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN

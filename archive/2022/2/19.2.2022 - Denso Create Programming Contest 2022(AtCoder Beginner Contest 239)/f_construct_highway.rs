//{"name":"F - Construct Highway","group":"AtCoder - Denso Create Programming Contest 2022(AtCoder Beginner Contest 239)","url":"https://atcoder.jp/contests/abc239/tasks/abc239_f","interactive":false,"timeLimit":2000,"tests":[{"input":"6 2\n1 2 1 2 2 2\n2 3\n1 4\n","output":"6 2\n5 6\n4 5\n"},{"input":"5 1\n1 1 1 1 4\n2 3\n","output":"-1\n"},{"input":"4 0\n3 3 3 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FConstructHighway"}}}

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let mut d = input.vec::<i64>(n);
    if d.iter().sum::<i64>() != (n - 1) as i64 * 2 {
        out_line!("-1");
        return;
    }
    let mut dsu = Dsu::new(n);
    for _ in 0..m {
        let x = input.usize() - 1;
        let y = input.usize() - 1;
        d[x] -= 1;
        d[y] -= 1;
        dsu.unite(x, y);
    }
    if d.iter().any(|&v| v < 0) {
        out_line!("-1");
        return;
    }
    let mut comps = dsu.calc_components();
    comps.sort_by_cached_key(|comp| {
        let mut res = 0i64;
        for &x in comp.iter() {
            res += d[x];
        }
        -res
    });
    let mut free = vec![];
    for &v in comps[0].iter() {
        for _ in 0..d[v] {
            free.push(v);
        }
    }
    let mut res = vec![];
    for comp in &comps[1..] {
        let mut cur_free = vec![];
        for &v in comp.iter() {
            for _ in 0..d[v] {
                cur_free.push(v);
            }
        }
        if cur_free.is_empty() || free.is_empty() {
            out_line!("-1");
            return;
        }
        let v1 = free.pop().unwrap();
        let v2 = cur_free.pop().unwrap();
        res.push((v1 + 1, v2 + 1));
        free.extend(&cur_free);
    }
    if res.len() != n - m - 1 {
        out_line!(-1);
        return;
    }
    for &res in res.iter() {
        out_line!(res);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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

//{"name":"F2. Чекер для Перемешивания массива","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/F2","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n2 1\n1 2\n4\n1 2 3 3\n3 3 2 1\n2\n2 1\n2 1\n4\n1 2 3 3\n3 2 3 1\n","output":"AC\nAC\nWA\nWA\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F2ChekerDlyaPeremeshivaniyaMassiva"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn has_cycle(g: &[Vec<usize>]) -> bool {
    let n = g.len();
    let mut seen = vec![0; n];
    for v in 0..n {
        if seen[v] == 2 {
            continue;
        }
        assert_eq!(seen[v], 0);
        if RecursiveFunction::new(|f, v: usize| {
            seen[v] = 1;
            for &to in g[v].iter() {
                if seen[to] == 1 {
                    return true;
                }
                if seen[to] == 2 {
                    continue;
                }
                if f.call(to) {
                    return true;
                }
            }
            seen[v] = 2;
            return false;
        })
        .call(v)
        {
            return true;
        }
    }
    false
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let b = input.vec::<usize>(n).sub_from_all(1);
    let mut g = vec![vec![]; n];
    for i in 0..n {
        g[a[i]].push(b[i]);
    }
    let mut max_id = 0;
    for i in 1..n {
        if g[i].len() > g[max_id].len() {
            max_id = i;
        }
    }
    g[max_id].clear();
    for i in 0..n {
        let mut iter = 0;
        while iter != g[i].len() {
            if g[i][iter] == max_id {
                g[i].swap_remove(iter);
            } else {
                iter += 1;
            }
        }
    }
    if has_cycle(&g) {
        out_line!("WA");
        return;
    }
    out_line!("AC");
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

//{"name":"J. Spy","group":"Codeforces - The 2019 ICPC Asia Nanjing Regional Contest","url":"https://codeforces.com/gym/103466/problem/J","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 2 3 4\n1 1 1 1\n0 0 1 1\n0 1 1 2\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JSpy"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::flows::hungarian_algorithm::hungarian_algorithm;
use algo_lib::flows::min_cost_max_flow::MinCostMaxFlow;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn with_flow(a: &Array2D<i64>) -> i64 {
    let n = a.len() - 1;
    let mut flow = MinCostMaxFlow::new(1 + n + n + 1);
    for i in 0..n {
        flow.add_edge(0, 1 + i, 1, 0);
        flow.add_edge(1 + i + n, 1 + n + n, 1, 0);
        for j in 0..n {
            flow.add_edge(1 + i, 1 + n + j, 1, a[i + 1][j + 1]);
        }
    }
    flow.find_min_cost_max_flow(0, 1 + n + n).cost
}

fn stress() {
    const MAX: usize = 50;
    for it in 1.. {
        let mut rnd = Random::new(787788 + it);
        dbg!(it);
        let n = rnd.gen_in_range(1..MAX);
        let mut a = Array2D::new(0, n + 1, n + 1);
        for i in 0..n {
            for j in 0..n {
                a[i][j] = -rnd.gen_in_range(0..1000);
            }
        }
        let fast = hungarian_algorithm(&a);
        let slow = with_flow(&a);
        if slow != fast {
            dbg!(n);
            for i in 0..n {
                for j in 0..n {
                    dbg!(i, j, a[i + 1][j + 1]);
                }
            }
            dbg!(slow);
            dbg!(fast);
        }
        assert_eq!(fast, slow);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let score = input.vec::<i64>(n);
    let b = input.vec::<i64>(n);
    let c = input.vec::<i64>(n);
    let mut matrix = Array2D::new(0, n, n);
    for i in 0..n {
        for j in 0..n {
            let sum = b[i] + c[j];
            let mut add_score = 0;
            for k in 0..n {
                if a[k] < sum {
                    add_score += score[k];
                }
            }
            matrix[i][j] = -add_score;
        }
    }
    let res = hungarian_algorithm(&matrix);
    out_line!(res * -1);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
    // tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN

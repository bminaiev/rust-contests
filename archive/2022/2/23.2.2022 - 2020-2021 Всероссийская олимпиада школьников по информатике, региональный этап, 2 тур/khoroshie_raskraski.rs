//{"name":"7. Хорошие раскраски","group":"Codeforces - 2020-2021 Всероссийская олимпиада школьников по информатике, региональный этап, 2 тур","url":"https://codeforces.com/gym/102936/problem/7?locale=en","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2 2\n","output":"1 2\n2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KhoroshieRaskraski"}}}

use std::ops::{Range, RangeBounds};

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_task(n: usize, m: usize, max_color: usize) -> Array2D<usize> {
    let mut res = Array2D::new(1, n, m);
    let calc = |a: &Array2D<usize>, check_xs: Range<usize>, check_ys: Range<usize>| -> usize {
        let mut res = 0;
        for x1 in 0..n {
            for x2 in x1 + 1..n {
                if !check_xs.contains(&x1) && !check_xs.contains(&x2) {
                    continue;
                }
                for y1 in 0..m {
                    if y1 >= check_ys.end {
                        break;
                    }
                    let y2_range = if check_ys.contains(&y1) {
                        y1 + 1..m
                    } else {
                        check_ys.clone()
                    };
                    for y2 in y2_range {
                        if a[x1][y1] == a[x1][y2]
                            && a[x1][y1] == a[x2][y1]
                            && a[x1][y1] == a[x2][y2]
                            && y1 != y2
                        {
                            res += 1;
                        }
                    }
                }
            }
        }
        res
    };

    let mut rnd = Random::new(8988899);
    for x in 0..n {
        for y in 0..m {
            res[x][y] = rnd.gen_in_range(1..max_color + 1);
        }
    }
    let mut sa = SimulatedAnnealing::new(0.9, SearchFor::MinimumScore, 10.0, 0.1);
    while sa.should_continue() {
        let x = rnd.gen_in_range(0..n);
        let y = rnd.gen_in_range(0..m);
        let c = rnd.gen_in_range(1..max_color + 1);

        let prev_score = calc(&res, x..x + 1, y..y + 1);
        let old_c = res[x][y];
        res[x][y] = c;
        let new_score = calc(&res, x..x + 1, y..y + 1);
        if !sa.should_go(prev_score, new_score) {
            res[x][y] = old_c;
        }
    }
    assert_eq!(calc(&res, 0..n, 0..n), 0);
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let max_color = input.usize();
    let res = solve_task(n, m, max_color);
    out_line!(res);
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
    // tester::run_single_test("1");
}
//END MAIN

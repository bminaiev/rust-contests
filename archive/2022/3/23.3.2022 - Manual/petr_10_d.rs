//{"name":"petr_10_d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"file","fileName":"domination.in","pattern":null},"output":{"type":"file","fileName":"domination.out","pattern":null},"languages":{"java":{"taskClass":"petr_10_d"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::SHIFTS_4;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn is02(x: i32, y: i32) -> bool {
    if x == 0 && y == 2 {
        true
    } else if x == 2 && y == 0 {
        true
    } else {
        false
    }
}

type Point = PointT<i32>;

fn calc_score(a: &Array2D<i32>) -> usize {
    let mut res = 0;
    let n = a.len();
    let m = a[0].len();
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 1 {
                res += 2;
            } else if a[i][j] == 0 {
                let mut ok = false;
                for s in SHIFTS_4.iter() {
                    let p = Point {
                        x: i as i32,
                        y: j as i32,
                    };
                    let next = p.apply_shift(s);
                    if next.index_arr2d(&a) == Some(&1) {
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    res += 1;
                }
            }
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = 10;
    let m = 10;
    let mut a = Array2D::new(0, n, m);
    let mut sa = SimulatedAnnealing::new(1.0, SearchFor::MinimumScore, 10.0, 0.01);
    let mut prev_score = calc_score(&a);
    let mut rnd = Random::new(78988);
    while sa.should_continue() {
        let x = rnd.gen_in_range(0..n);
        let y = rnd.gen_in_range(0..m);
        a[x][y] ^= 1;
        let new_score = calc_score(&a);
        if !sa.should_go(prev_score, new_score) {
            a[x][y] ^= 1;
        } else {
            prev_score = new_score;
        }
    }
    for i in 0..n {
        for j in 0..m {
            out!(a[i][j]);
        }
        out_line!();
    }

    /*let mut v = vec![];
    let mut res = 0;
    RecursiveFunction::new(|f, mut cnt02: usize| {
        if v.len() >= 2 {
            if is02(v[v.len() - 1], v[v.len() - 2]) {
                cnt02 += 1;
            }
        }
        if cnt02 > 1 {
            return;
        }
        if v.len() == n {
            res += 1;
        } else {
            for x in 0..=2 {
                v.push(x);
                f.call(cnt02);
                v.pop();
            }
        }
    })
    .call(0);
    dbg!(res);*/
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
        input: TaskIoType::File("domination.in".to_string()),
        output: TaskIoType::File("domination.out".to_string()),
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

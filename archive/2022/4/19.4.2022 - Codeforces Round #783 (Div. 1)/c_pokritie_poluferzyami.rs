//{"name":"C. Покрытие полуферзями","group":"Codeforces - Codeforces Round #783 (Div. 1)","url":"https://codeforces.com/contest/1667/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n","output":"1\n1 1\n"},{"input":"2\n","output":"1\n1 1\n"},{"input":"3\n","output":"2\n1 1\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPokritiePoluferzyami"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<usize>;

fn solve_n(n: usize) -> Vec<Point> {
    let mut res = vec![];
    for x in 1.. {
        let covers = x + (x + 1) / 2;
        if covers >= n {
            let half = x / 2;
            for i in 0..half {
                let delta = (x + 1) % 2;
                res.push(Point::new(i, (half - 1 - i + delta) % half));
            }
            for i in 0..x - half {
                res.push(Point::new(half + i, x - 1 - i));
            }
            return res;
        }
    }
    unreachable!();
}

fn stress() {
    let mut prev_res = 0;
    for n in 1.. {
        let mut attack = Array2D::new(0, n, n);
        let mut best = Array2D::new(false, n, n);
        let mut cur = Array2D::new(false, n, n);
        let mut res = prev_res + 1;
        RecursiveFunction::new(|f, cnt| {
            if cnt >= res {
                return;
            }
            let mut any_non_covered = false;
            for i in 0..n {
                for j in 0..n {
                    if attack[i][j] == 0 {
                        any_non_covered = true;
                    }
                }
            }
            if !any_non_covered {
                res = cnt;
                for i in 0..n {
                    for j in 0..n {
                        best[i][j] = cur[i][j];
                    }
                }
                return;
            }
            for ai in 0..n {
                for aj in 0..n {
                    let mut smth_new = false;
                    for i in 0..n {
                        for j in 0..n {
                            if ok(i, j, ai, aj) {
                                attack[i][j] += 1;
                                if attack[i][j] == 1 {
                                    smth_new = true;
                                }
                            }
                        }
                    }
                    if smth_new {
                        cur[ai][aj] = true;
                        f.call(cnt + 1);
                        cur[ai][aj] = false;
                    }
                    for i in 0..n {
                        for j in 0..n {
                            if ok(i, j, ai, aj) {
                                attack[i][j] -= 1;
                            }
                        }
                    }
                }
            }
        })
        .call(0);
        dbg!(n, res);
        for i in 0..n {
            for j in 0..n {
                if best[i][j] {
                    out!(1);
                } else {
                    out!(0);
                }
            }
            out_line!();
        }
        output().flush();
        prev_res = res + 1;
    }
}

fn ok(i: usize, j: usize, i1: usize, j1: usize) -> bool {
    if i == i1 || j == j1 {
        return true;
    }
    if i + 100 - i1 == j + 100 - j1 {
        return true;
    }
    return false;
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let res = solve_n(n);
    out_line!(res.len());
    for p in res.into_iter() {
        out_line!(p.x + 1, p.y + 1);
    }
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
    // tester::run_stress(stress);
}
//END MAIN

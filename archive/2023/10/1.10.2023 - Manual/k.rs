//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"k"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    pos: usize,
    value: i32,
}

struct Solver {
    a: Vec<i32>,
    b: Vec<i32>,
    res: Vec<i32>,
    take: f64,
    top: Vec<Pos>,
}

const MX: i32 = 1e9 as i32;

impl Solver {
    fn new(a: Vec<i32>, b: Vec<i32>, q: usize, max_ops: usize) -> Self {
        let res = vec![0; q + 1];
        let mut take = 1.0;
        loop {
            let up = (a.len() as f64 * take);
            let down = ((q + b.len()) as f64) * take;
            if up * down > max_ops as f64 {
                take -= 1e-3;
            } else {
                break;
            }
        }
        dbg!(take);
        let mut top = vec![];
        for i in 0..a.len() {
            top.push(Pos {
                pos: i,
                value: a[i],
            });
        }
        top.sort_by_key(|t| -t.value);
        let leave = if take == 1.0 {
            top.len()
        } else {
            (top.len() as f64 * take) as usize
        };
        top.truncate(leave);
        top.sort_by_key(|t| t.pos);

        let mut r = Self {
            a,
            b,
            res,
            take,
            top,
        };
        for i in 0..r.b.len() {
            r.calc_number(i, r.b[i]);
        }
        r
    }

    fn calc_number(&mut self, pos: usize, value: i32) {
        if self.take != 1.0 {
            let take_sz = (self.take * MX as f64) as i32;
            let take_from = MX - take_sz;
            if value < take_from {
                return;
            }
        }
        for top in self.top.iter() {
            if top.pos <= pos {
                let time = pos - top.pos;
                if time < self.res.len() {
                    self.res[time].update_max(value + top.value);
                }
            }
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let a = input.vec::<i32>(n);
    let b = input.vec::<i32>(n);
    let mut solver = Solver::new(a, b, q, 1e8 as usize);
    let mut res = solver.res[0];
    out_line!(res);
    for q_it in 0..q {
        let v = input.i32() ^ res;
        solver.calc_number(n + q_it, v);
        res = solver.res[q_it + 1];
        out_line!(res);
    }
}

fn stress() {
    let n = 500;
    let q = 1_000_000;
    let mut rnd = Random::new(787788);
    let a = rnd.gen_vec(n, 1..MX);
    let b = rnd.gen_vec(n, 1..MX);
    let mut sol1 = Solver::new(a.clone(), b.clone(), q, 1e8 as usize);
    let mut sol2 = Solver::new(a.clone(), b.clone(), q, 5e8 as usize);
    for q_it in 0..q {
        // dbg!(q_it);
        let v = rnd.gen(1..MX);
        sol1.calc_number(n + q_it, v);
        sol2.calc_number(n + q_it, v);
        let res1 = sol1.res[q_it];
        let res2 = sol2.res[q_it];
        assert_eq!(res1, res2);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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

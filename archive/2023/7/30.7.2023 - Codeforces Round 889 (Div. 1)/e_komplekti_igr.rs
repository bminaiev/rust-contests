//{"name":"E. Комплекты игр","group":"Codeforces - Codeforces Round 889 (Div. 1)","url":"https://codeforces.com/contest/1854/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n","output":"4\n20 20 20 20\n"},{"input":"722\n","output":"15\n15 14 13 12 11 10 9 8 7 6 5 4 3 2 1\n"},{"input":"1\n","output":"1\n60\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKomplektiIgr"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

const NEED: usize = 60;

struct Solver {
    ways: [i64; NEED + 1],
    numbers: Vec<usize>,
}

impl Solver {
    pub fn new() -> Self {
        let mut ways = [0; NEED + 1];
        ways[0] = 1;
        Self {
            ways,
            numbers: vec![],
        }
    }

    fn add_number(&mut self, x: usize) {
        assert!(x > 0);
        self.numbers.push(x);
        for i in (0..=NEED - x).rev() {
            self.ways[i + x] += self.ways[i];
        }
    }

    fn debug(&self) {
        for i in 0..=NEED {
            dbg!(i, self.ways[i]);
        }
    }
}

const MAX_NUMBERS: usize = 60;

fn find_sol(n: i64, iter: u64) -> Option<Solver> {
    let mut rnd = Random::new(787788 + iter);
    let mut solver = Solver::new();
    let cnt_numbers = rnd.gen(1..40);
    let max_number = rnd.gen(2..10);
    for _ in 0..cnt_numbers {
        solver.add_number(rnd.gen(1..max_number));
    }
    while solver.ways[NEED] < n && solver.numbers.len() < MAX_NUMBERS {
        let mut best_pos = 0;
        let mut best_delta = 0;
        for pos in 0..60 {
            if solver.ways[pos] + solver.ways[NEED] <= n {
                if solver.ways[pos] > best_delta {
                    best_delta = solver.ways[pos];
                    best_pos = pos;
                }
            }
        }
        solver.add_number(NEED - best_pos);
    }
    if solver.ways[NEED] != n {
        return None;
    }
    assert!(solver.numbers.len() <= MAX_NUMBERS);
    Some(solver)
}

fn find_sol_tries(n: i64) -> Solver {
    for it in 1.. {
        if let Some(sol) = find_sol(n, it) {
            return sol;
        }
    }
    unreachable!();
}

fn stress() {
    let mut rnd = Random::new(787788);
    for x in 1..=10000 {
        let n = rnd.gen_u64() % 10_000_000_000;
        let solver = find_sol_tries(n as i64);
        dbg!("Found", n);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let x = input.i64();
    let solver = find_sol_tries(x);
    out_line!(solver.numbers.len());
    for x in solver.numbers {
        out!(x, "");
    }
    out_line!();
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

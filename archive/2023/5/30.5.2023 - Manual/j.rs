//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use std::collections::BTreeMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Solver {
    green: usize,
    yellow: usize,
    red: usize,

    delta: BTreeMap<usize, i32>,
}

fn add_segm(fr: usize, to: usize, delta: &mut BTreeMap<usize, i32>) {
    *delta.entry(fr).or_default() += 1;
    *delta.entry(to).or_default() -= 1;
}

impl Solver {
    fn sum(&self) -> usize {
        self.green + self.yellow + self.red
    }
    fn add(&mut self, t: usize, c: u8) {
        let sum = self.sum();
        let mut t = t % sum;
        let mut len = self.green;
        if c == b'r' {
            len = self.red;
            t = (t + sum - self.green - self.yellow) % sum;
        } else if c == b'g' {
            len = self.green;
        } else if c == b'y' {
            len = self.yellow;
            t = (t + sum - self.green) % sum;
        } else {
            assert!(false);
        }
        if len == 0 {
            return;
        }
        let from = (t + sum + 1 - len) % sum;
        if from <= t {
            add_segm(from, t + 1, &mut self.delta);
        } else {
            add_segm(from, sum, &mut self.delta);
            add_segm(0, t + 1, &mut self.delta);
        }
    }

    fn calc_ok_len(&self, n: i32) -> usize {
        let mut sum = 0;
        let mut ok_len = 0;
        let mut prev_k = 0;
        for (&k, &v) in self.delta.iter() {
            if sum == n {
                ok_len += k - prev_k;
            }
            sum += v;
            prev_k = k;
        }
        ok_len
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let green = input.usize();
    let yellow = input.usize();
    let red = input.usize();
    let mut solver = Solver {
        green,
        red,
        yellow,
        delta: BTreeMap::new(),
    };
    let n = input.i32();
    let sum = green + yellow + red;
    assert!(sum > 0);

    let mut add = |solver: &mut Solver| {
        let t = input.usize() % sum;
        let c = input.string()[0];
        solver.add(t, c);
    };

    for _ in 0..n {
        add(&mut solver);
    }

    let ok_len = solver.calc_ok_len(n);
    if ok_len == 0 {
        out_line!(0.0);
    } else {
        assert!(ok_len > 0);
        add(&mut solver);
        let new_ok_len = solver.calc_ok_len(n + 1);
        let res = (new_ok_len as f64) / (ok_len as f64);
        out_line!(res);
    }
}

fn stress() {
    const TIME: usize = 100;
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(787788);
        const N: usize = 100;
        let r = rnd.gen(0..N);
        let g = rnd.gen(0..N);
        let y = rnd.gen(0..N);
        let sum = r + g + y;
        if sum == 0 {
            continue;
        }

        let start = rnd.gen(0..TIME);

        dbg!(start, g, y, r);

        let mut solver = Solver {
            green: g,
            yellow: y,
            red: r,
            delta: BTreeMap::new(),
        };

        let events = rnd.gen(1..5);
        for _ in 0..events {
            let time = rnd.gen(start..start + TIME);
            let shift = (time - start) % sum;
            let c = if shift < g {
                b'g'
            } else if shift < g + y {
                b'y'
            } else {
                b'r'
            };
            dbg!(time, c as char);
            solver.add(time, c);
        }
        let ok_len = solver.calc_ok_len(events);
        assert!(ok_len > 0);
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
    // tester::run_single_test("4");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN

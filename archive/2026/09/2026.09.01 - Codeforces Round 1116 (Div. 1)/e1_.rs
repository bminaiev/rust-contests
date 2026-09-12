#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let q = input.usize();
        let a = input.vec::<i64>(n);
        let mut histories = gen_vec(n, |i| SlowHistory::new(a[i]));
        let mut last_ans = 0;
        for time in 1..=q {
            let ty = input.usize() - 1;
            let d = |x: u64| -> usize { ((x ^ last_ans) % (n as u64)) as usize };
            if ty == 0 {
                let u = input.u64();
                let v = input.u64();
                let l = d(u).min(d(v));
                let r = d(u).max(d(v));
                let x = input.i64();
                for i in l..r + 1 {
                    histories[i].set(time, x);
                }
            } else if ty == 1 {
                let u = input.u64();
                let v = input.u64();
                let l = d(u).min(d(v));
                let r = d(u).max(d(v));
                for i in l..r + 1 {
                    histories[i].rev(time);
                }
            } else if ty == 2 {
                let u = input.u64();
                let v = input.u64();
                let l = d(u).min(d(v));
                let r = d(u).max(d(v));
                for i in l..r + 1 {
                    histories[i].max0(time);
                }
            } else {
                let u = input.u64();
                let pos = d(u);
                let res = histories[pos].get_answer(time);
                out.println(res);
                // TODO: check overflow?
                last_ans = res as u64;
            }
        }
    }
}

struct SlowHistory {
    a: Vec<i64>,
}

impl SlowHistory {
    fn new(start: i64) -> Self {
        Self { a: vec![start] }
    }

    fn set(&mut self, time: usize, value: i64) {
        self.ensure_time(time);
        self.a.push(value);
    }

    fn rev(&mut self, time: usize) {
        self.ensure_time(time);
        let last = *self.a.last().unwrap();
        self.a.push(-last);
    }

    fn max0(&mut self, time: usize) {
        self.ensure_time(time);
        let last = *self.a.last().unwrap();
        self.a.push(last.max(0));
    }

    fn ensure_time(&mut self, time: usize) {
        let last = *self.a.last().unwrap();
        while self.a.len() < time {
            self.a.push(last);
        }
    }

    fn get_answer(&mut self, time: usize) -> i64 {
        self.ensure_time(time);
        let mut res = i64::MIN;
        for i in 0..time {
            let mut sum = 0;
            for j in i..time {
                sum += self.a[j];
                res = res.max(sum);
            }
        }
        res
    }
}

#[derive(Clone, Copy)]
struct LinearFn {
    // ax + b
    // x >= 0
    // a == 0 or b == 0
    a: i64,
    b: i64,
}

#[derive(Clone, Copy)]
struct FastHistory {
    last_value: LinearFn,
    last_from_time: usize,
    // first LinearFn is always Ax + 0,
    // second LinearFn is always 0x + B
    max_segm: [LinearFn; 2],
    max_suf: [LinearFn; 2],
}

const NEG_INF: i64 = -1e18 as i64;

impl FastHistory {
    fn new(coef: i64) -> Self {
        assert!(coef.abs() == 1);
        Self {
            last_value: LinearFn { a: coef, b: 0 },
            last_from_time: 0,
            max_segm: [LinearFn { a: NEG_INF, b: 0 }, LinearFn { a: 0, b: NEG_INF }],
            max_suf: [LinearFn { a: 0, b: 0 }, LinearFn { a: 0, b: NEG_INF }],
        }
    }
}

fn stress() {
    const MAX_V: i64 = 10;
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let max_time = rnd.gen_range(2..10);
        let start_value = rnd.gen_range(-MAX_V..MAX_V);
        let mut slow = SlowHistory::new(start_value);
        let p = rnd.gen_double();
        for time in 1..=max_time {
            if rnd.gen_double() < p {
                continue;
            }
            let ty = rnd.gen_range(0..4);
            if ty == 0 {
                let value = rnd.gen_range(-MAX_V..MAX_V);
                slow.set(time, value);
            } else if ty == 1 {
                slow.rev(time);
            } else if ty == 2 {
                slow.max0(time);
            } else {
                let res = slow.get_answer(time);
                dbg!(time, res);
            }
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "e1_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}

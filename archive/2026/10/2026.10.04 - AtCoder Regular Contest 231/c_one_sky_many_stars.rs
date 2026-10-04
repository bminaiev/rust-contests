#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

struct State {
    a: Vec<i64>,
    d: Vec<i64>,
    me_ok: Vec<i32>,
    total_ok: i32,
    glob_fail: i32,
}

impl State {
    fn new(a: Vec<i64>, d: Vec<i64>) -> Self {
        let n = a.len();
        let me_ok = vec![0; n];
        let mut r = Self {
            a,
            d,
            me_ok,
            total_ok: 0,
            glob_fail: 0,
        };
        for i in 0..n - 1 {
            r.add_pair(i);
        }
        r.me_ok[0] += 1;
        if r.me_ok[0] == 1 {
            r.total_ok += 1;
        }
        r.me_ok[n - 1] += 1;
        if r.me_ok[n - 1] == 1 {
            r.total_ok += 1;
        }
        r
    }

    fn is_ok(&self) -> bool {
        self.total_ok == self.a.len() as i32 && self.glob_fail == 0
    }

    fn add_pair(&mut self, pos: usize) {
        let delta = self.a[pos + 1] - self.a[pos];
        if self.d[pos] + self.d[pos + 1] <= delta {
            self.me_ok[pos] += 1;
            self.me_ok[pos + 1] += 1;
            if self.me_ok[pos] == 1 {
                self.total_ok += 1;
            }
            if self.me_ok[pos + 1] == 1 {
                self.total_ok += 1;
            }
        }
        if self.d[pos] == delta + self.d[pos + 1] {
            self.me_ok[pos] += 1;
            if self.me_ok[pos] == 1 {
                self.total_ok += 1;
            }
        }
        if self.d[pos + 1] == delta + self.d[pos] {
            self.me_ok[pos + 1] += 1;
            if self.me_ok[pos + 1] == 1 {
                self.total_ok += 1;
            }
        }
        if self.d[pos] > delta + self.d[pos + 1] {
            self.glob_fail += 1;
        }
        if self.d[pos + 1] > delta + self.d[pos] {
            self.glob_fail += 1;
        }
    }

    fn rem_pair(&mut self, pos: usize) {
        let delta = self.a[pos + 1] - self.a[pos];
        if self.d[pos] + self.d[pos + 1] <= (self.a[pos + 1] - self.a[pos]) {
            self.me_ok[pos] -= 1;
            self.me_ok[pos + 1] -= 1;
            if self.me_ok[pos] == 0 {
                self.total_ok -= 1;
            }
            if self.me_ok[pos + 1] == 0 {
                self.total_ok -= 1;
            }
        }
        if self.d[pos] == delta + self.d[pos + 1] {
            self.me_ok[pos] -= 1;
            if self.me_ok[pos] == 0 {
                self.total_ok -= 1;
            }
        }
        if self.d[pos + 1] == delta + self.d[pos] {
            self.me_ok[pos + 1] -= 1;
            if self.me_ok[pos + 1] == 0 {
                self.total_ok -= 1;
            }
        }
        if self.d[pos] > delta + self.d[pos + 1] {
            self.glob_fail -= 1;
        }
        if self.d[pos + 1] > delta + self.d[pos] {
            self.glob_fail -= 1;
        }
    }

    fn update(&mut self, pos: usize, nd: i64) {
        if pos > 0 {
            self.rem_pair(pos - 1);
        }
        if pos + 1 < self.a.len() {
            self.rem_pair(pos);
        }
        self.d[pos] = nd;
        if pos > 0 {
            self.add_pair(pos - 1);
        }
        if pos + 1 < self.a.len() {
            self.add_pair(pos);
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<i64>(n);
        let d = input.vec::<i64>(n);
        let mut state = State::new(a, d);
        let q = input.usize();
        for _ in 0..q {
            let p = input.usize() - 1;
            let nd = input.i64();
            state.update(p, nd);
            if state.is_ok() {
                out.println("Yes");
            } else {
                out.println("No");
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
    const PROBLEM_NAME: &str = "c_one_sky_many_stars";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
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

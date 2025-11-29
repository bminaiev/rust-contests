//{"name":"F. Надо тренироваться","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n","output":"4\n1 4\n3 1\n2 1\n3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let mut sim = Simulator::new(n);
    solve_case(&mut sim, n);
    let missions = sim.all_missions.clone();
    out.println(missions.len());
    for (lvl, incr) in missions {
        out.println(vec![lvl, incr]);
    }
}

struct Simulator {
    can: Vec<bool>,
    cost: usize,
    last_mission: usize,
    all_missions: Vec<(usize, usize)>,
}

impl Simulator {
    fn new(n: usize) -> Self {
        Self {
            can: vec![true; n + 1],
            cost: 0,
            last_mission: usize::MAX,
            all_missions: vec![],
        }
    }

    fn do_mission(&mut self, lvl: usize, incr: usize) {
        if lvl == 0 {
            return;
        }
        self.all_missions.push((lvl, incr));
        assert!(self.can[lvl], "Cannot do mission at level {}", lvl);
        self.can[lvl] = false;
        let next = lvl + incr;
        if next < self.can.len() {
            self.can[next] = true;
        }
        self.cost += incr;
        if self.last_mission < lvl {
            self.cost += 1000;
        }
        self.last_mission = lvl;
    }

    fn check(&self) {
        for i in 1..self.can.len() {
            assert!(!self.can[i]);
        }
        assert!(self.cost <= 1_000_000);
    }

    fn print(&self) {
        let mut str = String::new();
        for i in 1..self.can.len() {
            if self.can[i] {
                str.push('1');
            } else {
                str.push('0');
            }
        }
        // dbg!(str);
        dbg!(self.cost);
        let cnt_ones = self.can.iter().filter(|&&x| x).count();
        dbg!(cnt_ones);
    }
}

fn solve_case(sim: &mut Simulator, n: usize) {
    // sim.print();
    const MAX_REM: usize = 65;
    let n = n + 1;
    for rem in 0..MAX_REM {
        for lvl in (rem..n).step_by(MAX_REM).rev() {
            sim.do_mission(lvl, 1);
        }
    }
    // sim.print();
    // dbg!("!!!");
    for rem in 0..MAX_REM {
        for lvl in (rem * MAX_REM..n).step_by(MAX_REM * MAX_REM).rev() {
            sim.do_mission(lvl, MAX_REM);
        }
    }
    // sim.print();
    // dbg!("!!!");
    for rem in 0..MAX_REM {
        for lvl in (rem * MAX_REM * MAX_REM..n)
            .step_by(MAX_REM * MAX_REM * MAX_REM)
            .rev()
        {
            sim.do_mission(lvl, MAX_REM * MAX_REM);
        }
    }
    // sim.print();
    // for x in 0..n {
    //     if sim.can[x] {
    //         dbg!(x);
    //     }
    // }
    sim.check();
}

fn stress() {
    let n = 250_000;
    let mut sim = Simulator::new(n);
    solve_case(&mut sim, n);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f_";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}

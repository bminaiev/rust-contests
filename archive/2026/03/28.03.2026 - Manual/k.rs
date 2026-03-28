//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type Point = PointT<i64>;

struct Solver {
    pos: Vec<Point>,
    baits: Vec<Point>,
    baits_dead: Vec<i32>,
    lives: Vec<i32>,
    extra_t: i64,
    dead_at: Vec<i64>,
    targets: Vec<usize>,
    cur_time: i64,
}

impl Solver {
    fn new(pos: Vec<Point>, baits: Vec<Point>, n_lives: usize, extra_t: i64) -> Self {
        let baits_dead = vec![0; baits.len()];
        let lives = vec![n_lives as i32; pos.len()];
        let dead_at = vec![i64::MAX; pos.len()];
        let targets = vec![usize::MAX; pos.len()];
        Self {
            pos,
            baits,
            baits_dead,
            lives,
            extra_t,
            dead_at,
            targets,
            cur_time: 0,
        }
    }

    fn update_target(&mut self, id: usize) {
        if !self.is_alive(id) {
            self.targets[id] = usize::MAX;
            return;
        }
        if self.targets[id] != usize::MAX && self.baits_dead[self.targets[id]] == 0 {
            // same target
            return;
        }
        let ids: Vec<usize> = (0..self.baits.len())
            .filter(|&i| self.baits_dead[i] == 0)
            .collect();
        let dist_to_me = |i: usize| self.pos[id].dist_manh(&self.baits[i]);
        if ids.is_empty() {
            self.targets[id] = usize::MAX;
        } else {
            let min = ids
                .into_iter()
                .min_by_key(|&id| (dist_to_me(id), self.baits[id].x, self.baits[id].y));
            self.targets[id] = min.unwrap().to_owned();
        }
    }

    fn simulate(&mut self) {
        for i in 0..self.pos.len() {
            self.update_target(i);
        }
        loop {
            let mut next_t = i64::MAX;
            for i in 0..self.pos.len() {
                if self.targets[i] == usize::MAX || !self.is_alive(i) {
                    continue;
                }
                let t = self.pos[i].dist_manh(&self.baits[self.targets[i]]) + self.cur_time;
                if t < next_t {
                    next_t = t;
                }
            }
            if next_t == i64::MAX {
                break;
            }
            // dbg!(self.cur_time, next_t);
            let delta_t = next_t - self.cur_time;
            self.cur_time = next_t;
            // go
            for i in 0..self.pos.len() {
                self.move_to(i, delta_t);
            }
            // eat
            for i in 0..self.pos.len() {
                // dbg!(
                //     i,
                //     self.pos[i],
                //     self.baits[self.targets[i]],
                //     self.is_alive(i),
                //     self.baits_dead[self.targets[i]]
                // );
                if !self.is_alive(i) || self.targets[i] == usize::MAX {
                    continue;
                }
                if self.pos[i] == self.baits[self.targets[i]]
                    && self.baits_dead[self.targets[i]] == 0
                {
                    self.lives[i] -= 1;
                    if self.lives[i] == 0 {
                        self.dead_at[i] = next_t + self.extra_t;
                    }
                    self.baits_dead[self.targets[i]] = 1;
                }
            }
            // choose new targets
            for i in 0..self.pos.len() {
                self.update_target(i);
            }
        }
    }

    fn move_to(&mut self, me: usize, mut dt: i64) {
        if self.targets[me] == usize::MAX {
            return;
        }
        let my_pos = self.pos[me];
        let target_pos = self.baits[self.targets[me]];
        if my_pos.x != target_pos.x {
            let max_x_dist = (target_pos.x - my_pos.x).abs();
            let use_x = max_x_dist.min(dt);
            self.pos[me].x += use_x * (target_pos.x - my_pos.x).signum();
            dt -= use_x;
        }
        if my_pos.y != target_pos.y {
            let max_y_dist = (target_pos.y - my_pos.y).abs();
            let use_y = max_y_dist.min(dt);
            self.pos[me].y += use_y * (target_pos.y - my_pos.y).signum();
            dt -= use_y;
        }
        assert!(dt >= 0);
    }

    fn is_alive(&self, id: usize) -> bool {
        self.dead_at[id] >= self.cur_time
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    let n_lives = input.usize();
    let extra_t = input.i64();
    let _w = input.i64();
    let _h = input.i64();
    let mut pos = vec![];
    for _ in 0..n {
        let x = input.i64();
        let y = input.i64();
        pos.push(Point::new(x, y));
    }
    let mut baits = vec![];
    for _ in 0..m {
        let x = input.i64();
        let y = input.i64();
        baits.push(Point::new(x, y));
    }
    let mut solver = Solver::new(pos, baits, n_lives, extra_t);
    solver.simulate();
    for x in solver.dead_at {
        if x == i64::MAX {
            out.println(-1);
        } else {
            out.println(x);
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
    const PROBLEM_NAME: &str = "k";
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

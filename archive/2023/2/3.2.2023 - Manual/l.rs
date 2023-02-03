//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Hash, PartialEq, Eq)]
struct PlayerState {
    hp: Vec<i32>,
    iter: usize,
}

impl PlayerState {
    pub fn alive(&self) -> bool {
        for &hp in self.hp.iter() {
            if hp > 0 {
                return true;
            }
        }
        false
    }
    pub fn cnt_alive(&self) -> usize {
        let mut res = 0;
        for &hp in self.hp.iter() {
            if hp > 0 {
                res += 1;
            }
        }
        res
    }

    pub fn new(a: &[i32]) -> Self {
        Self {
            hp: a.to_owned(),
            iter: 0,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    players: [PlayerState; 2],
    rev: bool,
}

#[derive(Clone, Copy)]
struct Res {
    first_win: f64,
    draw: f64,
}

fn calc(s: &State, hm: &mut HashMap<State, Res>, attacks: &[Vec<i32>]) -> Res {
    if let Some(r) = hm.get(s) {
        return r.clone();
    }
    let alive1 = s.players[0].alive();
    let alive2 = s.players[1].alive();
    if !alive1 && !alive2 {
        return Res {
            first_win: 0.0,
            draw: 1.0,
        };
    }
    if !alive1 {
        return Res {
            first_win: 0.0,
            draw: 0.0,
        };
    }
    if !alive2 {
        return Res {
            first_win: 1.0,
            draw: 0.0,
        };
    }
    let mut iter = s.players[0].iter;
    while s.players[0].hp[iter] == 0 {
        iter = iter + 1;
        if iter == s.players[0].hp.len() {
            iter = 0;
        }
    }
    let mut res = Res {
        first_win: 0.0,
        draw: 0.0,
    };
    let cnt_second = s.players[1].cnt_alive();
    let prob_here = 1.0 / (cnt_second as f64);
    let next_iter = (iter + 1) % s.players[0].hp.len();
    let (a1, a2) = if s.rev {
        (&attacks[1], &attacks[0])
    } else {
        (&attacks[0], &attacks[1])
    };
    for i in 0..s.players[1].hp.len() {
        if s.players[1].hp[i] > 0 {
            let mut next = State {
                players: [s.players[1].clone(), s.players[0].clone()],
                rev: !s.rev,
            };
            next.players[1].iter = next_iter;

            next.players[0].hp[i] -= a1[iter];
            next.players[0].hp[i].update_max(0);

            next.players[1].hp[iter] -= a2[i];
            next.players[1].hp[iter].update_max(0);

            let go = calc(&next, hm, attacks);
            res.draw += go.draw * prob_here;
            res.first_win += (1.0 - go.draw - go.first_win) * prob_here;
        }
    }
    hm.insert(s.clone(), res);
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.vec::<i32>(n);
    let b = input.vec::<i32>(m);

    let r1 = calc(
        &State {
            players: [PlayerState::new(&a), PlayerState::new(&b)],
            rev: false,
        },
        &mut HashMap::new(),
        &[a.clone(), b.clone()],
    );

    let r2 = {
        let r = calc(
            &State {
                players: [PlayerState::new(&b), PlayerState::new(&a)],
                rev: false,
            },
            &mut HashMap::new(),
            &[b.clone(), a.clone()],
        );
        Res {
            first_win: 1.0 - r.first_win - r.draw,
            draw: r.draw,
        }
    };

    let res = if a.len() > b.len() {
        r1
    } else if a.len() < b.len() {
        r2
    } else {
        Res {
            first_win: (r1.first_win + r2.first_win) / 2.0,
            draw: (r1.draw + r2.draw) / 2.0,
        }
    };
    out_line!(res.first_win);
    out_line!(1.0 - res.first_win - res.draw);
    out_line!(res.draw);
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

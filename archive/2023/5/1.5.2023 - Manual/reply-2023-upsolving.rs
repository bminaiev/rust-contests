//{"name":"reply-2023","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"reply-2023"}}}

use std::collections::BTreeSet;
use std::fs::OpenOptions;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_global_output_to_stdout};
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
use algo_lib::misc::simulated_annealing_mt::{simulated_annealing_mt, SaState};
use algo_lib::{dbg, out, out_line};
use marathon_utils::distribution_stat::DistributionStat;
use marathon_utils::hashcode_solver::{hashcode_solver, OneTest};
use rayon::prelude::*;
use std::io::prelude::*;

#[derive(Copy, Clone)]
enum Cell {
    Value(i64),
    Hole,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Snake {
    positions: Vec<Pos>,
}

impl Snake {
    fn new(positions: Vec<Pos>) -> Self {
        Self { positions }
    }
}

#[derive(Clone)]
struct State {
    snakes: Vec<Snake>,
    used: Array2D<bool>,
    score: i64,
}

const SZ: usize = 6;

#[derive(Clone, Debug)]
struct Replace {
    stay_idx: usize,
    new_pos: Vec<Pos>,
    rem_start: usize,
    rem_center: usize,
}

#[derive(Clone, Debug)]
enum ChangeInfo {
    Replace(Replace),
}

#[derive(Clone, Debug)]
struct Change {
    snake_id: usize,
    info: ChangeInfo,
    score_delta: i64,
}

impl State {
    fn new(t: &Task, snakes: Vec<Snake>) -> Self {
        let mut used = Array2D::new(false, t.rows, t.cols);
        let mut score = 0;
        for s in snakes.iter() {
            for p in s.positions.iter() {
                used[p.x][p.y] = true;
                score += t.get_cost(*p);
            }
        }
        Self {
            snakes,
            used,
            score,
        }
    }

    fn can_apply(&self, change: &Change) -> bool {
        if change.score_delta < -100000 {
            return false;
        }
        match &change.info {
            ChangeInfo::Replace(rem_end) => {
                let removes_end = rem_end.rem_center + rem_end.stay_idx + 1
                    == self.snakes[change.snake_id].positions.len();
                let sub = if removes_end { 0 } else { 1 };
                for p in rem_end.new_pos[..rem_end.new_pos.len() - sub].iter() {
                    if self.used[p.x][p.y] {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn remove_snake_pos(&mut self, snake_id: usize, pos: usize) {
        let p = self.snakes[snake_id].positions[pos];
        self.used[p.x][p.y] = false;
        self.snakes[snake_id].positions.remove(pos);
    }

    fn add_snake_pos(&mut self, snake_id: usize, idx: usize, p: Pos) {
        assert!(!self.used[p.x][p.y]);
        self.used[p.x][p.y] = true;
        self.snakes[snake_id].positions.insert(idx, p);
    }

    fn apply(&mut self, change: &Change) {
        self.score += change.score_delta;
        let snake_id = change.snake_id;
        let prev_len = self.snakes[snake_id].positions.len();
        match &change.info {
            ChangeInfo::Replace(rep) => {
                for _ in 0..rep.rem_center {
                    self.remove_snake_pos(snake_id, rep.stay_idx + 1);
                }
                for p in rep.new_pos.iter().rev() {
                    self.add_snake_pos(snake_id, rep.stay_idx + 1, *p);
                }
                for _ in 0..rep.rem_start {
                    self.remove_snake_pos(snake_id, 0);
                }
                assert_eq!(self.snakes[snake_id].positions.len(), prev_len);
            }
        }
    }
}

impl SaState for State {
    type Change = Change;

    type Score = i64;

    fn apply(&mut self, change: &Self::Change) {
        self.apply(change)
    }

    fn change_score_delta(&self, change: &Self::Change) -> Self::Score {
        change.score_delta
    }
}

struct Task {
    rows: usize,
    cols: usize,
    lens: Vec<usize>,
    a: Array2D<Cell>,
    nei: Array2D<[Pos; 4]>,
}

impl Task {
    fn get_cost(&self, p: Pos) -> i64 {
        match self.a[p.x][p.y] {
            Cell::Value(v) => v,
            Cell::Hole => i64::MIN / 1_000_000_0,
        }
    }

    fn nei(&self, p: Pos) -> &[Pos; 4] {
        &self.nei[p.x][p.y]
    }

    fn is_nei(&self, p1: Pos, p2: Pos) -> bool {
        for &p3 in self.nei[p1.x][p1.y].iter() {
            if p3 == p2 {
                return true;
            }
        }
        false
    }
}

fn solve_task(t: &Task, load_snakes: Vec<Vec<Pos>>) -> State {
    let mut snakes = vec![];
    if load_snakes.is_empty() {
        let mut path = vec![];
        for x in 0..t.rows {
            if x % 2 == 0 {
                for y in 0..t.cols {
                    path.push(Pos { x, y });
                }
            } else {
                for y in (0..t.cols).rev() {
                    path.push(Pos { x, y });
                }
            }
        }
        let mut path_cost = vec![];
        for p in path.iter() {
            path_cost.push(t.get_cost(*p));
        }
        let pref_sum = path_cost.pref_sum();
        let mut rnd = Random::new(787788);

        let mut bad_positions = BTreeSet::new();
        bad_positions.insert(path.len());
        for &len in t.lens.iter() {
            let mut best_pos = usize::MAX;
            let mut best_score = 0;
            for _ in 0..1000 {
                let pos = rnd.gen(0..path.len());
                let next_bad = bad_positions.range(pos..).next();
                if let Some(&next_bad) = next_bad {
                    if pos + len > next_bad {
                        continue;
                    }
                }
                let cur_score = pref_sum[pos + len] - pref_sum[pos];
                if cur_score >= best_score {
                    best_score = cur_score;
                    best_pos = pos;
                }
            }
            if best_pos != usize::MAX {
                assert_ne!(best_pos, usize::MAX);
                let mut used_positions = vec![];
                for i in 0..len {
                    used_positions.push(path[best_pos + i]);
                    bad_positions.insert(best_pos + i);
                }
                snakes.push(Snake::new(used_positions));
                // dbg!("SCORE:", best_score);
            } else {
                snakes.push(Snake::new(vec![]));
            }
        }
    } else {
        for i in 0..load_snakes.len() {
            snakes.push(Snake::new(load_snakes[i].clone()))
        }
    }

    let mut rnd = Random::new(787788);
    let mut random_deltas = vec![];
    for _ in 0..10_000 {
        let new_len = rnd.gen(1..SZ);
        let mut last = Pos { x: 0, y: 0 };
        let mut new_pos = Vec::with_capacity(new_len);
        for _ in 0..new_len {
            let next = t.nei(last)[rnd.gen(0..4)];
            new_pos.push(next);
            last = next;
        }
        let mut ok = true;
        for i in 0..new_len {
            for j in i + 1..new_len {
                if new_pos[i] == new_pos[j] {
                    ok = false;
                }
            }
        }
        if ok {
            random_deltas.push(new_pos);
        }
    }

    for snake in snakes.iter_mut() {
        if rnd.gen_bool() {
            snake.positions.reverse();
        }
    }

    let start_state = State::new(t, snakes);
    let start_score = start_state.score;

    let state = simulated_annealing_mt(
        60.0,
        start_state,
        start_score,
        |state: &State, rnd: &mut Random| {
            let snake_id: usize = rnd.gen(0..state.snakes.len());
            let positions = &state.snakes[snake_id].positions;
            let len = positions.len();
            let stay_idx = rnd.gen(0..len);
            let base_last = positions[stay_idx];
            for _cache_friendly_iter_try in 0..10 {
                let rd = &random_deltas[rnd.gen(0..random_deltas.len())];
                let mut last = base_last;
                let new_pos: Vec<_> = rd
                    .iter()
                    .map(|shift| {
                        let mut nx = last.x + shift.x;
                        let mut ny = last.y + shift.y;
                        if nx >= t.rows {
                            nx -= t.rows;
                        }
                        if ny >= t.cols {
                            ny -= t.cols;
                        }
                        Pos { x: nx, y: ny }
                    })
                    .collect();
                last = *new_pos.last().unwrap();
                let rem_center = rnd.gen(1..SZ);
                if stay_idx + rem_center >= len
                    || (positions[stay_idx + rem_center] != last
                        && stay_idx + rem_center != len - 1)
                    || rd.len() < rem_center
                {
                    continue;
                }

                let rem_start = new_pos.len() - rem_center;
                if rem_start > stay_idx {
                    continue;
                }
                let mut score_delta = 0;
                for i in 0..rem_start {
                    score_delta -= t.get_cost(positions[i]);
                }
                for i in 0..rem_center {
                    score_delta -= t.get_cost(positions[stay_idx + i + 1]);
                }
                for &p in new_pos.iter() {
                    score_delta += t.get_cost(p);
                }
                let new_change = Change {
                    snake_id,
                    info: ChangeInfo::Replace(Replace {
                        stay_idx,
                        new_pos,
                        rem_start,
                        rem_center,
                    }),
                    score_delta,
                };
                if state.can_apply(&new_change) {
                    return Some(new_change);
                }
            }
            return None;
        },
        SearchFor::MaximumScore,
        5.0,
        0.1,
    );

    let mut cnt_used = 0;
    let mut expected_score = 0;
    for x in 0..t.rows {
        for y in 0..t.cols {
            if state.used[x][y] {
                cnt_used += 1;
                expected_score += t.get_cost(Pos { x, y });
            }
        }
    }
    dbg!(cnt_used, expected_score, state.score);

    state
}

fn solve(input: &mut Input, test: &mut OneTest) {
    let cols = input.usize();
    let rows = input.usize();
    let n = input.usize();
    let lens = input.vec::<usize>(n);
    let mut a = Array2D::new(Cell::Hole, rows, cols);
    for i in 0..rows {
        for j in 0..cols {
            let token = input.string_as_string();
            if token != "*" {
                a[i][j] = Cell::Value(token.parse().unwrap())
            }
        }
    }

    test.report.add_value("rows", &rows);
    test.report.add_value("cols", &cols);
    test.report.add_value("cnt snakes", &n);

    {
        let mut ds = DistributionStat::new("snake lens");
        for &d in lens.iter() {
            ds.add(d as i32);
        }
        test.report.add_distribution_stat(&ds);
    }

    {
        let mut ds = DistributionStat::new("cell scores");
        for &d in a.iter() {
            if let Cell::Value(d) = d {
                ds.add(d as i32);
            }
        }
        test.report.add_distribution_stat(&ds);
    }

    let sum_lens: usize = lens.iter().sum();
    test.report.add_value("sum lens", &sum_lens);

    let mut num_holes = 0;
    for d in a.iter() {
        if let Cell::Hole = d {
            num_holes += 1;
        }
    }

    test.report.add_value("num holes", &num_holes);

    let mut all_values = vec![];
    for d in a.iter() {
        if let &Cell::Value(d) = d {
            if d > 0 {
                all_values.push(d);
            }
        }
    }
    all_values.sort();

    let from = if all_values.len() < sum_lens {
        0
    } else {
        all_values.len() - sum_lens
    };
    let potential_max_score: i64 = all_values[from..].iter().sum();

    test.report
        .add_value("cnt positive cells", &all_values.len());

    test.report
        .add_value("potential max score", &potential_max_score);

    let nei = Array2D::new_f(rows, cols, |x, y| {
        [
            Pos {
                x: (x + 1) % rows,
                y,
            },
            Pos {
                x,
                y: (y + 1) % cols,
            },
            Pos {
                x: (x + rows - 1) % rows,
                y,
            },
            Pos {
                x,
                y: (y + cols - 1) % cols,
            },
        ]
    });

    let t = Task {
        rows,
        cols,
        a,
        lens,
        nei,
    };
    let mut load_snakes = vec![];
    test.load_existing_result(|mut input: Input| {
        if !input.has_more_elements() {
            return;
        }
        for i in 0..t.lens.len() {
            let y = input.usize();
            let x = input.usize();
            let len = t.lens[i];
            let mut p = Pos { x, y };
            let mut res = vec![p];
            for j in 1..len {
                let token = input.string_as_string();
                if token == "D" {
                    p.x = (p.x + 1) % t.rows;
                } else if token == "U" {
                    p.x = (p.x + t.rows - 1) % t.rows;
                } else if token == "R" {
                    p.y = (p.y + 1) % t.cols;
                } else if token == "L" {
                    p.y = (p.y + t.cols - 1) % t.cols;
                } else {
                    input.string_as_string();
                    // assert!(false);
                }
                res.push(p);
                assert!(p.x < t.rows);
                assert!(p.y < t.cols);
            }
            load_snakes.push(res);
        }
    });
    let state = solve_task(&t, load_snakes);
    test.save_result(&mut || {
        for snake in state.snakes.iter() {
            if snake.positions.is_empty() {
                dbg!("EMPTY!");
                out_line!();
            } else {
                out!(snake.positions[0].y, snake.positions[0].x);
                for p in snake.positions.iter() {
                    if let Cell::Hole = t.a[p.x][p.y] {
                        assert!(false);
                    }
                }
                for i in 0..snake.positions.len() - 1 {
                    let p1 = snake.positions[i];
                    let p2 = snake.positions[i + 1];
                    if p1.x == p2.x {
                        if p1.y + 1 == p2.y || (p1.y == t.cols - 1 && p2.y == 0) {
                            out!(" R");
                        } else {
                            assert!(p2.y + 1 == p1.y || (p1.y == 0 && p2.y == t.cols - 1));
                            out!(" L");
                        }
                    } else {
                        assert!(p1.y == p2.y);
                        if p1.x + 1 == p2.x || (p1.x == t.rows - 1 && p2.x == 0) {
                            out!(" D");
                        } else {
                            assert!(p2.x + 1 == p1.x || (p1.x == 0 && p2.x == t.rows - 1));
                            out!(" U");
                        }
                    }
                }
                out_line!();
            }
        }
    });

    {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("res.txt")
            .unwrap();

        writeln!(file, "{}", state.score).unwrap();
    }

    // test.load_existing_result(|mut input: Input| {
    //     while input.has_more_elements() {
    //         perm.push(input.usize());
    //     }
    // });

    // let solutions: Vec<_> = (0..10)
    //     .into_par_iter()
    //     .map(|par_idx| {
    //         let mut rnd = Random::new(89799 + par_idx);

    //         let mut sa = SimulatedAnnealing::new(10.0, SearchFor::MaximumScore, 10.0, 0.001, 0);
    //         sa.with_out_prefix(format!("[{}] ", par_idx));
    //         while sa.should_continue() {
    //             if sa.should_go(new_score.score) {}
    //         }
    //     })
    //     .collect();

    // test.save_result(&mut || {
    //     for &x in perm.iter() {
    //         out_line!(x);
    //     }
    // });
}

pub(crate) fn run(mut _input: Input) -> bool {
    loop {
        hashcode_solver(
            &"reply-2023-upsolving",
            &"inputs",
            &"outputs",
            b'5'..=b'5',
            &mut solve,
        );
    }
    true
}

#[allow(unused)]
pub fn submit() {
    let sin = std::io::stdin();
    let input = Input::new(Box::new(sin));
    set_global_output_to_stdout();
    run(input);
}

//START MAIN
mod tester;

fn main() {
    tester::run_locally();
}
//END MAIN

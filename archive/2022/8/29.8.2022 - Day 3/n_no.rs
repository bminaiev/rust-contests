//{"name":"N. No!","group":"Yandex - Day 3","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39548/problems/N/","interactive":false,"timeLimit":3000,"tests":[{"input":"4 5\n3 5\n4 3\n2 5\n6 3\n5\n2\n3\n7\n2\n","output":"3/1\n3/2\n3/2\n1/0\n3/2\n"},{"input":"5 6\n10 6\n3 7\n6 15\n7 6\n8 15\n5\n3\n9\n7\n7\n9\n","output":"3/1\n3/1\n6/1\n3/1\n3/1\n6/1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NNo"}}}

use std::cmp::Ordering;
use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::frac::Frac;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Wall {
    h: i64,
    strength: i64,
}

#[derive(Clone, Copy)]
struct Query {
    id: usize,
    h: i64,
}

#[derive(Clone, Copy)]
enum Thing {
    Query(Query),
    Wall(Wall),
}

impl Thing {
    pub fn get_height(&self) -> i64 {
        match self {
            Thing::Query(q) => q.h,
            Thing::Wall(w) => w.h,
        }
    }

    pub fn get_sort_type(&self) -> i32 {
        match self {
            Thing::Query(_) => 0,
            Thing::Wall(_) => 1,
        }
    }
}

// at time [h], right idx should be removed, if both left & right is still alive
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    h: i64,
    left: usize,
    right: usize,
}

struct MyStack {
    walls: Vec<Wall>,
    alive: BTreeSet<usize>,
    events: BTreeSet<Event>,
}

impl MyStack {
    pub fn new() -> Self {
        Self {
            walls: vec![],
            alive: BTreeSet::new(),
            events: BTreeSet::new(),
        }
    }

    fn maybe_add_event(&mut self, right_idx: usize) {
        let w = self.walls[right_idx];
        if let Some(&prev) = self.alive.range(..right_idx).next_back() {
            let w_prev = self.walls[prev];

            let s1 = w_prev.strength;
            let h1 = w_prev.h;
            let s2 = w.strength;
            let h2 = w.h;

            assert_ne!(s1, s2);

            let h = (s1 * h2 - s2 * h1) / (s1 - s2);

            self.events.insert(Event {
                h,
                left: prev,
                right: right_idx,
            });
        }
    }

    pub fn add_wall(&mut self, w: Wall) {
        while !self.alive.is_empty() {
            let &last = self.alive.iter().next_back().unwrap();
            let prev_w = self.walls[last];
            if prev_w.h == w.h && prev_w.strength > w.strength {
                return;
            }
            if prev_w.strength <= w.strength {
                self.alive.remove(&last);
            } else {
                break;
            }
        }

        self.alive.insert(self.walls.len());
        self.walls.push(w);
        self.maybe_add_event(self.walls.len() - 1);
    }

    pub fn get_best(&mut self, h: i64) -> Frac {
        while !self.events.is_empty() {
            let ev = self.events.iter().next_back().unwrap().clone();
            if !self.alive.contains(&ev.left) || !self.alive.contains(&ev.right) {
                self.events.remove(&ev);
                continue;
            }
            if ev.h < h {
                break;
            }
            self.events.remove(&ev);
            self.alive.remove(&ev.right);
            if let Some(nxt) = self.alive.range(ev.right..).next() {
                self.maybe_add_event(*nxt);
            }
        }

        if let Some(&last_idx) = self.alive.iter().next_back() {
            let w = self.walls[last_idx];
            if w.h == h {
                return Frac::new(1, 0);
            }
            return Frac::new(w.strength, w.h - h);
        }

        Frac::new(0, 1)
    }
}

fn solve_case(walls: Vec<Wall>, queries: Vec<Query>) -> Vec<Frac> {
    let mut res = vec![Frac::new(0, 1); queries.len()];
    let mut things = vec![];
    for w in walls.into_iter() {
        things.push(Thing::Wall(w));
    }
    for q in queries.into_iter() {
        things.push(Thing::Query(q));
    }
    things.sort_by(|t1, t2| {
        t1.get_height()
            .cmp(&t2.get_height())
            .reverse()
            .then(t1.get_sort_type().cmp(&t2.get_sort_type()))
    });
    let mut dp = vec![Frac::new(0, 1); things.len()];
    for i in 0..things.len() {
        dp[i] = Frac::new(1, 0);
        if let Thing::Wall(_) = things[i] {
            break;
        }
    }

    let mut st_max = SegTreeMax::new(
        &MaxValNode {
            pos: 0,
            max_val: Frac::new(0, 1),
        },
        things.len(),
        (),
    );

    let mut stack = MyStack::new();

    let mut to_add: Vec<Vec<Wall>> = vec![vec![]; things.len() + 1];

    for i in 0..things.len() {
        for w in to_add[i].iter() {
            stack.add_wall(w.clone());
        }

        let mut cur_res = dp[i];
        {
            let from_pref_solver = st_max.get(i..i + 1).max_val;
            if cur_res.cmp(&from_pref_solver) == Ordering::Less {
                cur_res = from_pref_solver;
            }
        }
        {
            let from_suffix = stack.get_best(things[i].get_height());
            if cur_res < from_suffix {
                cur_res = from_suffix;
            }
        }

        match things[i] {
            Thing::Query(q) => res[q.id] = cur_res,
            Thing::Wall(w) => {
                let len = w.strength * cur_res.denum / cur_res.num;
                let till_incl = w.h - len;

                let pos_from = binary_search_first_true(0..things.len(), |check_pos| {
                    things[check_pos].get_height() < till_incl
                });
                st_max.update(0..pos_from, cur_res);
                to_add[pos_from].push(w);
            }
        }
    }
    res
}

fn solve_case_slow(walls: Vec<Wall>, queries: Vec<Query>) -> Vec<Frac> {
    let mut res = vec![Frac::new(0, 1); queries.len()];
    let mut things = vec![];
    for w in walls.into_iter() {
        things.push(Thing::Wall(w));
    }
    for q in queries.into_iter() {
        things.push(Thing::Query(q));
    }
    things.sort_by(|t1, t2| {
        t1.get_height()
            .cmp(&t2.get_height())
            .reverse()
            .then(t1.get_sort_type().cmp(&t2.get_sort_type()))
    });
    let mut dp = vec![Frac::new(0, 1); things.len()];
    for i in 0..things.len() {
        dp[i] = Frac::new(1, 0);
        if let Thing::Wall(_) = things[i] {
            break;
        }
    }
    for i in 0..things.len() {
        match things[i] {
            Thing::Query(q) => res[q.id] = dp[i],
            Thing::Wall(w) => {
                let cur_res = dp[i];
                let len = w.strength * cur_res.denum / cur_res.num;
                let till_incl = w.h - len;
                for j in i + 1..things.len() {
                    if things[j].get_height() >= till_incl {
                        if dp[j].cmp(&cur_res) == Ordering::Less {
                            dp[j] = cur_res;
                        }
                    } else {
                        let delta_h = w.h - things[j].get_height();
                        let next_res = Frac::new(w.strength, delta_h);
                        if dp[j].cmp(&next_res) == Ordering::Less {
                            dp[j] = next_res;
                        }
                    }
                }
            }
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let walls = gen_vec(n, |_| Wall {
        h: input.read(),
        strength: input.read(),
    });
    let queries = gen_vec(q, |id| Query {
        id,
        h: input.read(),
    });
    let res = solve_case(walls, queries);
    for r in res.into_iter() {
        let s = format!("{}/{}", r.num, r.denum);
        out_line!(s);
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

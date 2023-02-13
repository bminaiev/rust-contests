//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use std::cmp::max;
use std::time::Instant;

use algo_lib::collections::sqrt_decomposition::{Part, SqrtDecomposition, SqrtNode};
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn handle_prefix(sum_balance: i64) -> (i64, i64) {
    let mut res = 0;
    let mut cur_balance = 0;
    if sum_balance > 0 {
        let close = sum_balance;
        res += close;
        cur_balance -= close;
        res += close * (close + 1) / 2;
    } else {
        let open = -sum_balance;
        res += open;
    }
    (res, cur_balance)
}

fn solve_a(a: &[i64]) -> i64 {
    let mut sum_balance = 0;
    for &x in a.iter() {
        sum_balance += x;
    }
    let (mut res, mut cur_balance) = handle_prefix(sum_balance);
    for &x in a.iter() {
        cur_balance += x;
        if x < 0 {
            res += max(0, -cur_balance);
        }
    }
    res
}

const BLOCK_SIZE: usize = 100;

#[derive(Default, Clone, Debug)]
struct Solver {
    balance_delta: i64,
    pref_sum: Vec<i32>,
    pref_cnt: Vec<i32>,
}

impl Solver {
    pub fn from_raw(a: &[i64], mult: i64) -> Self {
        let mut balance = BLOCK_SIZE as i64;
        let mut cnt = [0i32; BLOCK_SIZE * 2 + 1];

        for &x in a.iter() {
            let x = x * mult;
            balance += x;
            if x < 0 {
                cnt[balance as usize] += 1;
            }
        }

        let mut pref_sum = vec![0; BLOCK_SIZE * 2 + 2];
        let mut pref_cnt = vec![0; BLOCK_SIZE * 2 + 2];
        for i in 0..cnt.len() {
            pref_sum[i + 1] = pref_sum[i] + cnt[i] * i as i32;
            pref_cnt[i + 1] = pref_cnt[i] + cnt[i];
        }

        Self {
            balance_delta: balance - BLOCK_SIZE as i64,
            pref_sum,
            pref_cnt,
        }
    }

    pub fn handle(&self, cur_balance: i64) -> (i64, i64) {
        let mut res = 0;

        let pos = -cur_balance + (BLOCK_SIZE as i64);
        if pos >= 0 {
            let mut pos = pos as usize;
            if pos >= self.pref_sum.len() {
                pos = self.pref_sum.len() - 1;
            }
            res += self.pref_cnt[pos] as i64 * (BLOCK_SIZE as i64 - cur_balance)
                - self.pref_sum[pos] as i64;
        }

        (cur_balance + self.balance_delta, res)
    }
}

#[test]
fn test() {
    let solver = Solver::from_raw(&[-1, -1], 1);
    dbg!(solver);
    // dbg!(solver.handle(0));
    dbg!(solver.handle(1));
    // dbg!(solver.handle(-2));
}

#[derive(Default, Clone)]
struct Node {
    rev: bool,
    solver: Solver,
    rev_solver: Solver,
}

impl SqrtNode for Node {
    type Value = i64;

    fn relax(&mut self, raw_values: &mut [Self::Value]) {
        if self.rev {
            for x in raw_values.iter_mut() {
                *x = -*x;
            }
        }
    }

    fn rebuild(&mut self, raw_values: &[Self::Value]) {
        self.solver = Solver::from_raw(raw_values, 1);
        self.rev_solver = Solver::from_raw(raw_values, -1);
        self.rev = false;
    }
}

#[derive(Default, Clone)]
struct SegTreeNode {
    len: i64,
    sum: i64,
}

impl LazySegTreeNodeSpec for SegTreeNode {
    fn unite(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        Self {
            len: l.len + r.len,
            sum: l.sum + r.sum,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        if *update == 1 {
            node.sum = -node.sum;
        }
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current ^= add;
    }

    type Update = i32;

    type Context = ();
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let start = Instant::now();
        let mut rnd = Random::new(787788 + it);
        let n = 150_000;
        let a = gen_vec(n, |_| if rnd.gen_bool() { 1 } else { -1 });
        let mut sqrt = SqrtDecomposition::new(a.clone(), BLOCK_SIZE, Node::default());
        let mut st = LazySegTree::<SegTreeNode>::new_f(n, &|pos| SegTreeNode {
            len: 1,
            sum: a[pos],
        });
        let mut xor_res = 0;
        for _ in 0..n {
            let range = rnd.gen_nonempty_range(n);
            let (l, r) = (range.start, range.end);
            if rnd.gen_bool() {
                sqrt.iter_mut(l..r, |part| match part {
                    Part::Full(full) => {
                        full.rev = !full.rev;
                    }
                    Part::Single(_block, value) => *value = -*value,
                });
                st.update(l..r, 1);
            } else {
                let sum_balance = st.get(l..r).sum;
                let (mut res, mut cur_balance) = handle_prefix(sum_balance);
                sqrt.iter_mut(l..r, |part| match part {
                    Part::Full(full) => {
                        let (nbalance, res_delta) = if full.rev {
                            full.rev_solver.handle(cur_balance)
                        } else {
                            full.solver.handle(cur_balance)
                        };
                        cur_balance = nbalance;
                        res += res_delta;
                    }
                    Part::Single(_block, &mut x) => {
                        cur_balance += x;
                        if x < 0 {
                            res += max(0, -cur_balance);
                        }
                    }
                });
                xor_res ^= res;
            }
        }
        dbg!(xor_res, start.elapsed());
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let s = input.string();
    let mut a = vec![1; n];
    for i in 0..n {
        if s[i] == b')' {
            a[i] = -1;
        }
    }
    let mut st = LazySegTree::<SegTreeNode>::new_f(n, &|pos| SegTreeNode {
        len: 1,
        sum: a[pos],
    });
    let mut sqrt = SqrtDecomposition::new(a.clone(), BLOCK_SIZE, Node::default());
    for _ in 0..q {
        let q_type = input.usize();
        let l = input.usize() - 1;
        let r = input.usize();
        if q_type == 1 {
            sqrt.iter_mut(l..r, |part| match part {
                Part::Full(full) => {
                    full.rev = !full.rev;
                }
                Part::Single(_block, value) => *value = -*value,
            });
            st.update(l..r, 1);
        } else {
            let sum_balance = st.get(l..r).sum;
            let (mut res, mut cur_balance) = handle_prefix(sum_balance);
            sqrt.iter_mut(l..r, |part| match part {
                Part::Full(full) => {
                    let (nbalance, res_delta) = if full.rev {
                        full.rev_solver.handle(cur_balance)
                    } else {
                        full.solver.handle(cur_balance)
                    };
                    cur_balance = nbalance;
                    res += res_delta;
                }
                Part::Single(_block, &mut x) => {
                    cur_balance += x;
                    if x < 0 {
                        res += max(0, -cur_balance);
                    }
                }
            });
            // let res2 = solve_a(&a[l..r]);
            out_line!(res)
        }
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

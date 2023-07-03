//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"a"}}}

use std::cmp::{max, min};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::lazy_seg_tree::{SegTree, SegTreeNode};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Segment {
    l: i32,
    r: i32,
    color: usize,
}

type Mod = Mod_998_244_353;

#[derive(Clone, Default)]
struct Node {
    sum: Mod,
    cnt: Mod,
}

// x -> k*x + b
#[derive(Clone)]
struct Update {
    k: Mod,
    b: Mod,
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        Self {
            sum: l.sum + r.sum,
            cnt: l.cnt + r.cnt,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.sum = node.sum * update.k + update.b * node.cnt;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        // (k1*x + b1)*k2 + b2 = k1*k2*x + b1*k2 + b2
        current.k = current.k * add.k;
        current.b = current.b * add.k + add.b;
    }

    type Update = Update;
    type Context = ();
}

fn solve_case_slow(segs: Vec<Segment>) -> Mod {
    let mut res = Mod::ZERO;
    for mask in 0..(1 << segs.len()) {
        let mut ok = true;
        for i in 0..segs.len() {
            for j in i + 1..segs.len() {
                if ((1 << i) & mask) != 0 {
                    if ((1 << j) & mask) != 0 {
                        if segs[i].color != segs[j].color {
                            if max(segs[i].l, segs[j].l) <= min(segs[i].r, segs[j].r) {
                                ok = false;
                            }
                        }
                    }
                }
            }
        }
        if ok {
            res += Mod::ONE;
        }
    }
    res
}

fn solve_case(mut segs: Vec<Segment>) -> Mod {
    segs.sort();
    let mut all_coords = vec![];
    for seg in &segs {
        all_coords.push(seg.l);
        all_coords.push(seg.r);
    }
    all_coords.sort();
    all_coords.dedup();

    let mut dp = vec![
        SegTree::<Node>::new(all_coords.len(), |_| Node {
            sum: Mod::ZERO,
            cnt: Mod::ONE,
        });
        2
    ];

    for seg in segs.iter() {
        let idx_l = all_coords.binary_search(&seg.l).unwrap();
        let idx_r = all_coords.binary_search(&seg.r).unwrap();

        let mut ways = Mod::ONE;
        ways += dp[seg.color].get(0..idx_r + 1).sum;
        ways += dp[1 - seg.color].get(0..idx_l).sum;

        dp[seg.color].update(
            idx_r..idx_r + 1,
            Update {
                k: Mod::ONE,
                b: ways,
            },
        );
        dp[seg.color].update(
            idx_r + 1..all_coords.len(),
            Update {
                k: Mod::TWO,
                b: Mod::ZERO,
            },
        );
    }
    let mut res = Mod::ONE;
    res += dp[0].get(0..all_coords.len()).sum;
    res += dp[1].get(0..all_coords.len()).sum;
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let segs = gen_vec(n, |_| Segment {
        l: input.i32(),
        r: input.i32(),
        color: input.usize(),
    });
    let res = solve_case(segs);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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

fn stress() {
    const N: usize = 10;
    for it in 7.. {
        dbg!(it);
        let mut rnd = Random::new(787788 + it);
        let n = rnd.gen(1..N);
        let mut segs = vec![];
        for _ in 0..n {
            let l = rnd.gen(1..10);
            let r = rnd.gen(l..11);
            let color = rnd.gen(0..2);
            segs.push(Segment { l, r, color });
        }
        let fast = solve_case(segs.clone());
        let slow = solve_case_slow(segs.clone());
        if fast != slow {
            dbg!(segs);
            dbg!(fast);
            dbg!(slow);
            panic!();
        }
    }
}

fn main() {
    tester::run_tests();
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN

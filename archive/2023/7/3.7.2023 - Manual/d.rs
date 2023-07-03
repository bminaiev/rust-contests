//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::kenetic_seg_tree_max::KeneticSegTreeMax;
use algo_lib::seg_trees::lazy_seg_tree::{SegTree, SegTreeNode};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone)]
enum Query {
    UpdateMin(i64),
    AddIndex,
    Sum(usize, usize),
}

#[derive(Clone, Default)]
struct Node {
    cnt_alive: i64,
    default_add: i64,
    sum: i64,
}

#[derive(Clone)]
struct Update {
    set_all_alive_to_value: Option<i64>,
    add_x_times: i64,
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        Self {
            cnt_alive: l.cnt_alive + r.cnt_alive,
            default_add: l.default_add + r.default_add,
            sum: l.sum + r.sum,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        if let Some(value) = update.set_all_alive_to_value {
            node.sum = value * node.cnt_alive;
        }
        node.sum += update.add_x_times * node.default_add;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        if add.set_all_alive_to_value.is_some() {
            *current = add.clone();
        } else {
            current.add_x_times += add.add_x_times;
        }
    }

    type Update = Update;
    type Context = ();
}

fn solve_case(n: usize, a: &[i64], queries: &[Query]) -> Vec<i64> {
    let mut seg_tree = KeneticSegTreeMax::new(n, |pos| ((pos + 1) as i64, a[pos]), 0);

    let mut before_mined_seg_tree = SegTree::<Node>::new(n, |pos| Node {
        cnt_alive: 1,
        default_add: (pos + 1) as i64,
        sum: a[pos],
    });
    let mut after_mined_seg_tree = SegTree::<Node>::new(n, |_pos| Node {
        cnt_alive: 0,
        default_add: 0,
        sum: 0,
    });
    let mut already_mined = BTreeSet::<usize>::new();

    let mut res = vec![];
    let mut time = 0;
    for query in queries.iter() {
        match query {
            &Query::UpdateMin(val) => {
                {
                    // binary search & min
                    let min_from = binary_search_first_true(0..n, |check_pos| {
                        let alive_pos = already_mined.range(check_pos..).next().copied();
                        if let Some(alive_pos) = alive_pos {
                            after_mined_seg_tree.get(alive_pos..alive_pos + 1).sum >= val
                        } else {
                            true
                        }
                    });
                    after_mined_seg_tree.update(
                        min_from..n,
                        Update {
                            set_all_alive_to_value: Some(val),
                            add_x_times: 0,
                        },
                    );
                }
                loop {
                    let max_line = seg_tree.get_max(0..n);
                    if max_line.get_value(time) < val {
                        break;
                    }
                    let pos = max_line.pos;
                    seg_tree.update(pos, 0, -std::i64::MAX);
                    before_mined_seg_tree.update_point(
                        pos,
                        Node {
                            cnt_alive: 0,
                            default_add: 0,
                            sum: 0,
                        },
                    );
                    after_mined_seg_tree.update_point(
                        pos,
                        Node {
                            cnt_alive: 1,
                            default_add: (pos + 1) as i64,
                            sum: val,
                        },
                    );
                    already_mined.insert(pos);
                }
            }
            Query::AddIndex => {
                time += 1;
                seg_tree.update_time(time);
                let update = Update {
                    set_all_alive_to_value: None,
                    add_x_times: 1,
                };
                before_mined_seg_tree.update(0..n, update.clone());
                after_mined_seg_tree.update(0..n, update);
            }
            &Query::Sum(l, r) => {
                let before_mined = before_mined_seg_tree.get(l..r).sum;
                let after_mined = after_mined_seg_tree.get(l..r).sum;
                res.push(before_mined + after_mined);
            }
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let a = input.vec::<i64>(n);
    let queries = gen_vec(q, |_| {
        let q_type = input.usize();
        match q_type {
            1 => Query::UpdateMin(input.i64()),
            2 => Query::AddIndex,
            3 => Query::Sum(input.usize() - 1, input.usize()),
            _ => unreachable!(),
        }
    });
    let res = solve_case(n, &a, &queries);
    for r in res {
        out_line!(r);
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

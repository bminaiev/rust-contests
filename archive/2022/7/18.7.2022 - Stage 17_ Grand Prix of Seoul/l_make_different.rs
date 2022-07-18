//{"name":"L. Make Different","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/L/","interactive":false,"timeLimit":3000,"tests":[{"input":"8 3\n1 2 2 2 1 2 1 2\n1 2\n1 5\n3 6\n","output":"0\n-1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LMakeDifferent"}}}

use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::group_by::GroupByTrait;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone)]
struct Group {
    expected_child: [usize; 2],
    members: BTreeSet<usize>,
}

impl Group {
    pub fn new(expected_child: [usize; 2]) -> Self {
        Self {
            expected_child,
            members: BTreeSet::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ExpectedChange {
    v: usize,
    old_group: usize,
    new_child: [usize; 2],
}

#[derive(Clone, Copy)]
struct GroupIdChange {
    v: usize,
    new_group_id: usize,
}

#[derive(Clone, Copy)]
struct Query {
    id: usize,
    v: usize,
    u: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let jump_dist = input.vec::<usize>(n);

    let mut queries_by_v = vec![vec![]; n];
    let mut res = vec![std::i32::MAX; q];
    for id in 0..q {
        let query = Query {
            id,
            v: input.usize() - 1,
            u: input.usize() - 1,
        };
        queries_by_v[query.v].push(query);
        queries_by_v[query.u].push(query);
        if jump_dist[query.v] != jump_dist[query.u] {
            res[query.id] = 0;
        }
    }

    let mut group_id = gen_vec(n, |i| jump_dist[i] - 1);
    let g = gen_vec(n, |pos| {
        [(pos + jump_dist[pos]) % n, (pos + n - jump_dist[pos]) % n]
    });

    let mut g_rev = vec![vec![]; n];
    for v in 0..n {
        for &to in g[v].iter() {
            g_rev[to].push(v);
        }
    }

    let mut groups = vec![Group::new([0, 0]); 2];
    for v in 0..n {
        groups[group_id[v]].members.insert(v);
    }

    let mut to_check: BTreeSet<_> = (0..n).collect();
    for step in 1.. {
        let get_change = |v: usize| ExpectedChange {
            v,
            old_group: group_id[v],
            new_child: [group_id[g[v][0]], group_id[g[v][1]]],
        };

        let mut changes: Vec<_> = to_check.iter().map(|&v| get_change(v)).collect();
        changes.sort_by_key(|c| (c.old_group, c.new_child));

        let mut group_id_changes = vec![];
        for changes in changes.group_by_(|c1, c2| c1.old_group == c2.old_group) {
            let mut changes = changes.to_vec();
            let cur_group = &mut groups[changes[0].old_group];
            if changes.len() * 2 > cur_group.members.len() {
                changes = cur_group.members.iter().map(|&v| get_change(v)).collect();
                changes.sort_by_key(|c| c.new_child);
                for same_new_group in changes.group_by_(|c1, c2| c1.new_child == c2.new_child) {
                    if same_new_group.len() * 2 > cur_group.members.len() {
                        cur_group.expected_child = same_new_group[0].new_child;
                    }
                }
            }
            let old_expected_child = cur_group.expected_child;

            for changes in changes.group_by_(|c1, c2| c1.new_child == c2.new_child) {
                let new_child = changes[0].new_child;
                if new_child == old_expected_child {
                    continue;
                }
                for c in changes.iter() {
                    group_id_changes.push(GroupIdChange {
                        v: c.v,
                        new_group_id: groups.len(),
                    });
                }
                groups.push(Group::new(new_child));
            }
        }

        to_check.clear();
        for change in group_id_changes.iter() {
            let v = change.v;

            groups[group_id[v]].members.remove(&v);
            group_id[v] = change.new_group_id;
            groups[group_id[v]].members.insert(v);

            for &prev in g_rev[change.v].iter() {
                to_check.insert(prev);
            }
        }

        for change in group_id_changes.iter() {
            for &query in queries_by_v[change.v].iter() {
                if group_id[query.v] != group_id[query.u] {
                    res[query.id].update_min(step);
                }
            }
        }

        if group_id_changes.is_empty() {
            break;
        }
    }
    for &r in res.iter() {
        if r == std::i32::MAX {
            out_line!(-1);
        } else {
            out_line!(r);
        }
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

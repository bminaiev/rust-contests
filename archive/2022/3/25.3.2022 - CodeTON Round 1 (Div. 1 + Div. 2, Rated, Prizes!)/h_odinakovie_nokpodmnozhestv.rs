//{"name":"H. Одинаковые НОК подмножеств","group":"Codeforces - CodeTON Round 1 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1656/problem/H","interactive":false,"timeLimit":10000,"tests":[{"input":"4\n3 4\n5 6 7\n2 8 9 10\n4 4\n5 6 7 8\n2 3 4 9\n1 3\n1\n1 2 3\n5 6\n3 4 9 7 8\n2 15 11 14 20 12\n","output":"NO\nYES\n1 2\n6\n2 3\nYES\n1 1\n1\n1\nYES\n3 2\n3 7 4\n12 14\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HOdinakovieNOKPodmnozhestv"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::{gcd, lcm};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Default, Copy, Debug)]
pub struct LcmNode {
    lcm: i128,
}

impl LazySegTreeNodeSpec for LcmNode {
    #[allow(unused)]
    fn unite(l: &Self, r: &Self, context: &()) -> Self {
        Self {
            lcm: lcm(l.lcm, r.lcm),
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.lcm = *update;
    }

    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }

    type Update = i128;
    type Context = ();
}

fn solve(input: &mut Input, _test_case: usize) {
    let sz = input.vec::<usize>(2);
    let a = gen_vec(2, |id| input.vec::<i128>(sz[id]));
    let mut trees = gen_vec(2, |id| {
        gen_vec(sz[id], |pos| {
            LazySegTree::new_f(sz[id ^ 1], &|npos| LcmNode {
                lcm: gcd(a[id][pos], a[id ^ 1][npos]),
            })
        })
    });
    let mut queue = vec![];
    let mut removed = gen_vec(2, |id| vec![false; sz[id]]);
    for i in 0..2 {
        for j in 0..sz[i] {
            if trees[i][j].get(0..sz[1 ^ i]).lcm != a[i][j] {
                queue.push((i, j));
                removed[i][j] = true;
            }
        }
    }
    while let Some((i, j)) = queue.pop() {
        for j2 in 0..sz[i ^ 1] {
            trees[i ^ 1][j2].update(j..j + 1, 1);
            if !removed[i ^ 1][j2] && trees[i ^ 1][j2].get(0..sz[i]).lcm != a[i ^ 1][j2] {
                removed[i ^ 1][j2] = true;
                queue.push((i ^ 1, j2));
            }
        }
    }
    let res = gen_vec(2, |id| {
        (0..sz[id])
            .filter_map(|pos| {
                if removed[id][pos] {
                    None
                } else {
                    Some(a[id][pos])
                }
            })
            .collect::<Vec<_>>()
    });
    if res[0].is_empty() || res[1].is_empty() {
        out_line!("NO");
        return;
    }
    out_line!("YES");
    out_line!(res[0].len(), res[1].len());
    out_line!(res[0]);
    out_line!(res[1]);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
}
//END MAIN

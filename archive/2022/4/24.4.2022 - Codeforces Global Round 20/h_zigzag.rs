//{"name":"H. Зигзаг","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/H","interactive":false,"timeLimit":1000,"tests":[{"input":"5 3\n11011\n2 4\n1 5\n3 5\n","output":"1\n3\n2\n"},{"input":"10 3\n1001110110\n1 10\n2 5\n5 10\n","output":"4\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HZigzag"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn sol(s: &[u8]) -> usize {
    let mut stack = vec![];
    let mut res = 1;
    for w in s.windows(2) {
        if w[0] != w[1] {
            continue;
        }
        let x = w[0];
        if stack.is_empty() || *stack.last_exn() == x {
            stack.push(x);
        } else {
            stack.pop();
            res += 1;
        }
    }
    res += stack.len();
    res
}

#[derive(Clone, Copy, Default)]
struct Node {
    res: i32,
    cnt: i32,
    value: u8,
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self, context: &Self::Context) -> Self {
        if l.value == r.value {
            Self {
                res: l.res + r.res,
                value: l.value,
                cnt: l.cnt + r.cnt,
            }
        } else {
            if l.cnt > r.cnt {
                Self {
                    res: l.res + r.res + r.cnt,
                    value: l.value,
                    cnt: l.cnt - r.cnt,
                }
            } else {
                Self {
                    res: l.res + r.res + l.cnt,
                    value: r.value,
                    cnt: r.cnt - l.cnt,
                }
            }
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        todo!()
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        todo!()
    }

    type Update = ();

    type Context = ();
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let s = input.string();

    if n == 1 {
        for _ in 0..q {
            input.usize();
            input.usize();
            out_line!(1);
        }
        return;
    }

    let mut seg_tree = LazySegTree::new_f(n - 1, &|pos| -> Node {
        if s[pos] == s[pos + 1] {
            Node {
                res: 0,
                value: s[pos],
                cnt: 1,
            }
        } else {
            Node {
                res: 0,
                value: 0,
                cnt: 0,
            }
        }
    });

    for _ in 0..q {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;

        if fr == to {
            out_line!(1);
            continue;
        }

        let res = seg_tree.get(fr..to);

        let ops = 1 + res.res + res.cnt;

        out_line!(ops);
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
}
//END MAIN

//{"name":"G. Fishingprince снова играет с массивом","group":"Codeforces - Codeforces Global Round 21","url":"https://codeforces.com/contest/1696/problem/G","interactive":false,"timeLimit":6000,"tests":[{"input":"4 3\n1 2\n3 1 1 4\n2 1 4\n1 1 1\n2 1 3\n","output":"3.500000000000000\n1.000000000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GFishingprinceSnovaIgraetSMassivom"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Default)]
struct Node {
    f: [[f64; 4]; 4],
    x: f64,
    y: f64,
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self, context: &Self::Context) -> Self {
        let mut res = Node {
            f: [[0.0; 4]; 4],
            x: l.x,
            y: r.y,
        };
        for use_mask1 in 0..4 {
            for use_mask2 in 0..4 {
                for join_mask1 in 0..4 {
                    for join_mask2 in 0..4 {
                        let n_use_mask = (use_mask1 & 1) | (use_mask2 & 2);
                        let n_join_mask = (join_mask1 & 1) | (join_mask2 & 2);
                        if (use_mask1 & 2) != 0 && (use_mask2 & 1) != 0 {
                            if (join_mask1 & 2) == 0 {
                                continue;
                            }
                            if (join_mask2 & 1) == 0 {
                                continue;
                            }
                        }
                        // if (join_mask1 & 2) != 0 && (use_mask2 & 1) != 0 && (join_mask2 & 1) == 0 {
                        //     continue;
                        // }
                        // if (join_mask2 & 1) != 0 && (use_mask1 & 2) != 0 && (join_mask1 & 2) == 0 {
                        //     continue;
                        // }
                        let nval = l.f[use_mask1][join_mask1] + r.f[use_mask2][join_mask2];
                        if nval > res.f[n_use_mask][n_join_mask] {
                            res.f[n_use_mask][n_join_mask] = nval;
                        }
                    }
                }
            }
        }

        res
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        for use_mask in 0..4 {
            for join_mask in 0..4 {
                let value = if use_mask != 3 { 0.0 } else { *update };
                let coef = if join_mask == 0 {
                    node.y
                } else {
                    node.x + node.y
                };
                node.f[use_mask][join_mask] = value / coef;
            }
        }
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }

    type Update = f64;

    type Context = ();
}

fn solve_case(a: &[f64], x: f64, y: f64) -> f64 {
    let mut dp = vec![0.0; a.len() + 2];
    for start in 0..a.len() {
        if dp[start + 1] < dp[start] {
            dp[start + 1] = dp[start];
        }
        {
            let nval = dp[start] + a[start] / y;
            if dp[start + 2] < nval {
                dp[start + 2] = nval
            }
        }
        let mut sum = 0.0;
        for end in start..a.len() {
            sum += a[end];
            let nval = dp[start] + sum / (x + y);
            if dp[end + 2] < nval {
                dp[end + 2] = nval;
            }
        }
    }
    let mut res = 0.0;
    for &x in dp.iter() {
        if x > res {
            res = x;
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let mut x = input.f64().0;
    let mut y = input.f64().0;
    if x > y {
        let tmp = x;
        x = y;
        y = tmp;
    }
    assert!(x <= y);
    let a = input.vec::<f64>(n);

    let mut st = LazySegTree::new_f(n, &|_pos| Node {
        f: [[0.0; 4]; 4],
        x,
        y,
    });
    for i in 0..n {
        st.update(i..i + 1, a[i]);
    }

    for _ in 0..q {
        if input.usize() == 1 {
            let pos = input.usize() - 1;
            // a[pos] = input.f64().0;
            st.update(pos..pos + 1, input.f64().0)
        } else {
            let l = input.usize() - 1;
            let r = input.usize();
            let node = st.get(l..r);
            let mut res = 0.0;
            for m1 in 0..4 {
                for m2 in 0..4 {
                    let v = node.f[m1][m2];
                    if v > res {
                        res = v;
                    }
                }
            }
            out_line!(res);
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

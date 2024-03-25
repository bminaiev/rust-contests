//{"name":"D - LIS on Tree 2","group":"AtCoder - AtCoder Regular Contest 175","url":"https://atcoder.jp/contests/arc175/tasks/arc175_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 8\n1 2\n2 3\n2 4\n4 5\n","output":"Yes\n3 2 1 4 5\n"},{"input":"7 21\n2 1\n7 2\n5 1\n3 7\n2 6\n3 4\n","output":"No\n"},{"input":"8 20\n3 1\n3 8\n7 1\n7 5\n3 2\n6 5\n4 7\n","output":"Yes\n2 1 3 5 6 8 4 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DLISOnTree2"}}}

use std::vec;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut k = input.i64() - n as i64;
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut sizes = vec![1; n];
    RecursiveFunction2::new(|f, v: usize, p: usize| {
        for &to in g[v].iter() {
            if to == p {
                continue;
            }
            f.call(to, v);
            sizes[v] += sizes[to];
        }
    })
    .call(0, 0);
    let mut next = vec![usize::MAX; n];
    RecursiveFunction2::new(|f, v: usize, p: usize| -> (usize, usize) {
        let mut sum_children = sizes[v] - 1;
        let mut cur_left = usize::MAX;
        let mut cur_right = usize::MAX;
        let mut joined = false;
        for &to in g[v].iter() {
            if to == p {
                continue;
            }
            if sum_children <= k && !joined {
                joined = true;
                k -= sum_children;
                if cur_right != usize::MAX {
                    next[cur_right] = v;
                }
                cur_right = v;
                if cur_left == usize::MAX {
                    cur_left = v;
                }
            }
            let (ch_left, ch_right) = f.call(to, v);
            if cur_right != usize::MAX {
                next[cur_right] = ch_left;
                cur_right = ch_right;
            } else {
                cur_left = ch_left;
                cur_right = ch_right;
            }
            sum_children -= sizes[to];
        }
        if !joined {
            if cur_right != usize::MAX {
                next[cur_right] = v;
            }
            cur_right = v;
            if cur_left == usize::MAX {
                cur_left = v;
            }
        }
        (cur_left, cur_right)
    })
    .call(0, 0);
    let mut prev = vec![usize::MAX; n];
    for i in 0..n {
        if next[i] != usize::MAX {
            prev[next[i]] = i;
        }
    }
    let mut roots = vec![];
    for i in 0..n {
        if next[i] == usize::MAX {
            roots.push(i);
        }
    }
    assert_eq!(roots.len(), 1);
    let start = roots[0];
    let mut res = vec![0; n];
    let mut cur = start;
    for i in (0..n).rev() {
        res[cur] = i + 1;
        cur = prev[cur];
    }
    if k == 0 {
        out.println("Yes");
        out.println(res);
    } else {
        out.println("No");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_lison_tree2";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

//{"name":"D. Сбалансированное дерево","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n4\n0 11\n6 6\n0 0\n5 5\n2 1\n3 1\n4 3\n7\n1 1\n0 5\n0 5\n2 2\n2 2\n2 2\n2 2\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n4\n1 1\n1 1\n1 1\n0 0\n1 4\n2 4\n3 4\n7\n0 20\n0 20\n0 20\n0 20\n3 3\n4 4\n5 5\n1 2\n1 3\n1 4\n2 5\n3 6\n4 7\n5\n1000000000 1000000000\n0 0\n1000000000 1000000000\n0 0\n1000000000 1000000000\n3 2\n2 1\n1 4\n4 5\n6\n21 88\n57 81\n98 99\n61 76\n15 50\n23 67\n2 1\n3 2\n4 3\n5 3\n6 4\n","output":"11\n3\n3\n5\n3000000000\n98\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSbalansirovannoeDerevo"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut g = vec![vec![]; n];
        for i in 0..n {
            left[i] = input.i64();
            right[i] = input.i64();
        }
        for _ in 0..n - 1 {
            let fr = input.usize() - 1;
            let to = input.usize() - 1;
            g[fr].push(to);
            g[to].push(fr);
        }
        let mut root = 0;
        while g[root].len() > 1 {
            root += 1;
        }
        let mut res = 0;
        let root_val = RecursiveFunction2::new(|f, v: usize, p: usize| -> i64 {
            let mut children = vec![];
            for &to in g[v].iter() {
                if to == p {
                    continue;
                }
                let ch = f.call(to, v);
                children.push(ch);
            }
            children.sort();
            if children.is_empty() {
                return left[v];
            }
            let target = children[children.len() - 1];
            let value = target.clamp(left[v], right[v]);
            for &x in children.iter() {
                if x > value {
                    res += x - value;
                }
            }
            value
        })
        .call(root, root);
        out.println(root_val + res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_sbalansirovannoe_derevo";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

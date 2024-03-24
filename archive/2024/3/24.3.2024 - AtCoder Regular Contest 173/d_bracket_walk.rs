//{"name":"D - Bracket Walk","group":"AtCoder - AtCoder Regular Contest 173","url":"https://atcoder.jp/contests/arc173/tasks/arc173_d","interactive":false,"timeLimit":3000,"tests":[{"input":"5 7\n1 2 (\n2 3 )\n3 4 (\n4 1 )\n2 4 )\n4 5 (\n5 1 )\n","output":"Yes\n"},{"input":"2 2\n1 2 )\n2 1 )\n","output":"No\n"},{"input":"10 20\n4 5 (\n5 6 (\n6 7 )\n2 5 )\n5 8 (\n6 3 )\n8 5 )\n1 2 (\n9 10 (\n4 7 (\n3 4 )\n8 9 (\n2 1 )\n1 4 )\n2 3 )\n3 2 (\n7 8 (\n7 4 )\n10 9 )\n9 8 )\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBracketWalk"}}}

use algo_lib::collections::bit_set::BitSet;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    balance: i32,
}

fn bfs(g: &[Vec<Edge>], max_balance: usize) -> Vec<BitSet> {
    let mut res = vec![BitSet::new(max_balance * 2 + 1); g.len()];
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, max_balance));
    res[0].set(max_balance, true);
    while let Some((v, balance)) = q.pop_front() {
        for &edge in &g[v] {
            let new_balance = balance as i32 + edge.balance;
            if new_balance >= 0 && new_balance <= max_balance as i32 * 2 {
                let new_balance = new_balance as usize;
                if !res[edge.to].get(new_balance) {
                    q.push_back((edge.to, new_balance));
                    res[edge.to].set(new_balance, true);
                }
            }
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut g = vec![vec![]; n];
    let mut g_rev = vec![vec![]; n];
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let balance = if input.string() == [b'('] { 1 } else { -1 };
        g[fr].push(Edge { to, balance });
        g_rev[to].push(Edge {
            to: fr,
            balance: -balance,
        })
    }
    let max_balance = n + 5;
    let forward = bfs(&g, max_balance);
    let backward = bfs(&g_rev, max_balance);
    let mut ok = true;
    for v in 0..n {
        for e in g[v].iter() {
            let mut edge_ok = false;
            for b1 in 1..max_balance * 2 - 1 {
                if forward[v].get(b1) {
                    let b2 = (b1 as i32 + e.balance) as usize;
                    if backward[e.to].get(b2) {
                        edge_ok = true;
                        break;
                    }
                }
            }
            if !edge_ok {
                ok = false;
                break;
            }
        }
    }
    if ok {
        out.println("Yes");
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
    const PROBLEM_NAME: &str = "d_bracket_walk";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

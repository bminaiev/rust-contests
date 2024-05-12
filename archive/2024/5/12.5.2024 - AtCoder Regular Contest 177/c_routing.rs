//{"name":"C - Routing","group":"AtCoder - AtCoder Regular Contest 177","url":"https://atcoder.jp/contests/arc177/tasks/arc177_c","interactive":false,"timeLimit":2500,"tests":[{"input":"5\nRBRBB\nRBRRR\nRRRBR\nRBBRB\nBBRBR\n","output":"3\n"},{"input":"5\nRBBBB\nBBBBB\nBBBBB\nBBBBB\nBBBBR\n","output":"7\n"},{"input":"10\nRRBBBBBBBB\nBRRBBBBBBB\nBBRRBBBBBB\nBBBRRBBBBB\nBBBBRRBBBB\nBBBBBRRBBB\nBBBBBBRRBB\nBBBBBBBRRB\nBBBBBBBBRR\nBBBBBBBBBR\n","output":"2\n"},{"input":"17\nRBBRRBRRRRRBBBBBB\nBBRBRBRRBRRBRRBBR\nBRBRBBBRBBRBBRBBB\nRBRRBBBBBBRRBRRRR\nRRRRRBRBRRRBBRBBR\nRRRRRBRRBRBBRRRBB\nBBBRRRBRBRBBRRRBB\nBBRRRBRBBBRBRRRBR\nRRBBBBBBBBBBBRBRR\nRRRBRRBRBRBRBRBBB\nRRBRRRRBRBRRBRBBR\nRRRBBRBRBBBRBBRBR\nBBRBBRRBRRRBBRBBB\nBBBRBRRRRRRRBBRBB\nRRRRRBRBRBBRRBRRR\nBRRRRBBBRRRBRRBBB\nBBRRBBRRRBBBRBBBR\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRouting"}}}

use std::collections::VecDeque;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;

fn cost(a: &[Vec<usize>]) -> usize {
    let n = a.len();
    assert_eq!(a[0][0], 0);
    assert_eq!(a[n - 1][n - 1], 0);
    let mut queue = VecDeque::new();
    let mut dist = Array2D::new(usize::MAX, n, n);
    queue.push_back((0usize, 0usize));
    dist[0][0] = 0;
    let deltas = [(0usize, 1usize), (1, 0), (0, !0), (!0, 0)];
    while let Some((i, j)) = queue.pop_front() {
        for (di, dj) in deltas.iter().copied() {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if ni < n && nj < n {
                let nd = dist[i][j] + a[ni][nj];
                if nd < dist[ni][nj] {
                    dist[ni][nj] = nd;
                    if a[ni][nj] == 1 {
                        queue.push_back((ni, nj));
                    } else {
                        queue.push_front((ni, nj));
                    }
                }
            }
        }
    }
    dist[n - 1][n - 1]
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut s = gen_vec(n, |_| {
        let s = input.string();
        let mut row = vec![0; n];
        for i in 0..n {
            row[i] = if s[i] == b'B' { 1 } else { 0 };
        }
        row
    });
    let mut res = cost(&s);
    s.reverse();
    for s in s.iter_mut() {
        for c in s.iter_mut() {
            *c = 1 - *c;
        }
    }
    res += cost(&s);
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c_routing";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

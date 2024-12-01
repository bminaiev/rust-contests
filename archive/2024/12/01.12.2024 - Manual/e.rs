//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy)]
struct Edge {
    nmask: usize,
    cost: i64,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    let a0 = input.i64();
    let a1 = input.i64();
    let a2 = input.i64();
    let a3 = input.i64();
    const N: usize = 16;
    const TOTAL_STATES: usize = 1 << N;
    const MX: i64 = i64::MAX / 10;
    let mut dp = vec![MX; TOTAL_STATES];
    let mut edges = vec![vec![]; TOTAL_STATES];
    const AND: usize = TOTAL_STATES - 1 - (1 << 15);
    for mask in 0..TOTAL_STATES {
        {
            // a0
            for i in 0..4 {
                let mut nmask = 0;
                for j in 0..N {
                    if (1 << j) & mask != 0 {
                        let nj = j ^ (1 << i);
                        nmask |= 1 << nj;
                    }
                }
                nmask &= AND;
                edges[mask].push(Edge { nmask, cost: a0 });
            }
        }
        {
            // a1
            for i in [3, 12] {
                let mut nmask = 0;
                for j in 0..N {
                    if (1 << j) & mask != 0 {
                        let nj = j ^ i;
                        nmask |= 1 << nj;
                    }
                }
                nmask &= AND;
                edges[mask].push(Edge { nmask, cost: a1 });
            }
        }
        {
            // a2
            for i in [5, 10] {
                let mut nmask = 0;
                for j in 0..N {
                    if (1 << j) & mask != 0 {
                        let nj = j ^ i;
                        nmask |= 1 << nj;
                    }
                }
                nmask &= AND;
                edges[mask].push(Edge { nmask, cost: a2 });
            }
        }
        {
            //
            for i in [15] {
                let mut nmask = 0;
                for j in 0..N {
                    if (1 << j) & mask != 0 {
                        let nj = j ^ i;
                        nmask |= 1 << nj;
                    }
                }
                nmask &= AND;
                edges[mask].push(Edge { nmask, cost: a3 });
            }
        }
    }
    dp[0] = 0;
    loop {
        let mut changed = false;
        for mask in 0..TOTAL_STATES {
            for e in &edges[mask] {
                if dp[e.nmask] + e.cost < dp[mask] {
                    dp[mask] = dp[e.nmask] + e.cost;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    for _ in 0..tc {
        let m = input.usize();
        let mut full_mask = 0;
        for _ in 0..m {
            let mut mask = 0;
            for i in 0..2 {
                let s = input.string();
                for j in 0..2 {
                    if s[j] == b'1' {
                        mask |= 1 << (i * 2 + j);
                    }
                }
            }
            assert!(mask < 16);
            // dbg!(mask);
            full_mask |= 1 << mask;
        }
        // dbg!(full_mask);
        out.println(dp[full_mask]);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "e";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

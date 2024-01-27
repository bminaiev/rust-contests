//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"k"}}}

use std::collections::VecDeque;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn stress() {
    const M: usize = 10_000;
    let mut seen = Array2D::new((usize::MAX, usize::MAX), M, M);
    let mut queue = VecDeque::new();
    queue.push_back((1, 0));
    queue.push_back((1, 1));
    seen[1][0] = (1, 0);
    seen[1][1] = (0, 0);
    // let mut first = vec![usize::MAX; M];
    while let Some((a0, a1)) = queue.pop_front() {
        // if first[a1] == usize::MAX {
        //     first[a1] = seen[a0][a1];
        //     // dbg!(a0, a1);
        // }
        for &(b0, b1) in [
            (a0 + a1, 0),
            (a0 + 2 * a1, a1),
            (a1, 0),
            (2 * a0 + 3 * a1, a0 + 2 * a1),
            (3 * a0 + 2 * a1, a0 + a1),
        ]
        .iter()
        {
            if b0 >= M || b1 >= M {
                continue;
            }
            if seen[b0][b1].0 == usize::MAX {
                seen[b0][b1] = (a0, a1);
                queue.push_back((b0, b1));
            }
        }
    }
    // for x in 0..first.len() / 3 {
    //     if first[x] > 10 {
    //         dbg!(x, first[x]);
    //     }
    // }
    for y in 0..500 {
        let sy = y;
        let mut x = y * 2;
        let mut y = y;
        let mut path = vec![];
        loop {
            path.push((x, y));
            let (px, py) = seen[x][y];
            if x == px && y == py {
                break;
            }
            x = px;
            y = py;
        }
        eprintln!("{sy}: {path:?}");
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "k";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use std::cmp::max;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let k = input.usize();
    let mut a = vec![];
    for _ in 0..k {
        a.push(Point {
            x: input.usize() - 1,
            y: input.usize() - 1,
        });
    }
    let mut at_least = Array2D::new(0, n, m);
    for r in 0..n {
        let s = input.string();
        for c in 0..m {
            if s[c] == b'#' {
                at_least[r][c] = usize::MAX;
            }
        }
    }
    for i in 1..k {
        // (k - 1) -> 0
        let first = k - i;
        at_least[a[i].x][a[i].y] = first;
    }
    let mut queues = vec![vec![]; 2 * n * m + 3];
    queues[0].push(a[0]);
    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];
    let mut seen = Array2D::new(false, n, m);
    seen[a[0].x][a[0].y] = true;
    let mut res = 0u64;
    for d in 0..queues.len() {
        let mut cur_queue = vec![];
        std::mem::swap(&mut queues[d], &mut cur_queue);
        for cur in cur_queue.iter() {
            let zz = (d as u64) * (d as u64);
            res = res.overflowing_add(zz).0;
            for i in 0..4 {
                let nx = cur.x as i32 + dx[i];
                let ny = cur.y as i32 + dy[i];
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if seen[nx][ny] {
                    continue;
                }
                let time = max(at_least[nx][ny], d + 1);
                if time != usize::MAX {
                    queues[time].push(Point { x: nx, y: ny });
                    seen[nx][ny] = true;
                }
            }
        }
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "g";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "3");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

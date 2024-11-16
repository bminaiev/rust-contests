//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use std::collections::HashMap;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Line {
    cnt: usize,
    coord: usize,
    val: i32,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut by_coord = vec![HashMap::<i32, Vec<usize>>::new(); 3];
        for i in 0..n {
            let p = [input.i32(), input.i32(), input.i32()];
            for j in 0..3 {
                by_coord[j].entry(p[j]).or_default().push(i);
            }
        }
        let mut lines = vec![];
        for i in 0..3 {
            for (val, coords) in by_coord[i].iter() {
                lines.push(Line {
                    cnt: coords.len(),
                    coord: i,
                    val: *val,
                });
            }
        }
        lines.sort();
        lines.reverse();
        let mut alive = vec![false; n];
        let mut cnt_alive = 0;
        let mut can_alive = vec![0; n + 1];
        for line in lines.iter() {
            for &i in by_coord[line.coord][&line.val].iter() {
                if !alive[i] {
                    alive[i] = true;
                    cnt_alive += 1;
                }
            }
            let dencity = line.cnt;
            can_alive[dencity] = cnt_alive;
        }
        for i in (0..can_alive.len() - 1).rev() {
            let z = can_alive[i + 1];
            can_alive[i] = z.max(can_alive[i]);
        }
        for i in 0..can_alive.len() {
            can_alive[i] = n - can_alive[i];
        }
        out.println(can_alive[1..].to_vec());
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "j";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

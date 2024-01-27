//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use std::collections::VecDeque;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

const K: usize = 12;
fn conv(mut a: usize) -> [usize; K] {
    let mut res = [0; K];
    for i in 0..K {
        res[i] = a % 3;
        a /= 3;
    }
    res
}

fn conv_back(a: [usize; K]) -> usize {
    let mut res = 0;
    for i in (0..K).rev() {
        res *= 3;
        res += a[i];
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);

    const MAX: usize = 531441;
    let mut exist = vec![false; MAX];
    for x in a.iter() {
        exist[*x] = true;
    }
    let mut seen = Array2D::new(false, 2, MAX);
    let mut queue = VecDeque::new();
    queue.push_back((true, a[0]));
    seen[1][a[0]] = true;
    while let Some((can, val)) = queue.pop_front() {
        let mut a = conv(val);
        if !can && exist[val] && !seen[1][val] {
            seen[1][val] = true;
            queue.push_back((true, val));
        }
        for bit in 0..K {
            let prev = a[bit];
            for nval in 0..3 {
                if nval == prev {
                    continue;
                }
                a[bit] = nval;
                let mut ncan = can;
                if nval < prev {
                    if !can {
                        continue;
                    }
                    ncan = false;
                }
                let nval = conv_back(a);
                if seen[ncan as usize][nval] {
                    continue;
                }
                seen[ncan as usize][nval] = true;
                queue.push_back((ncan, nval));
            }
            a[bit] = prev;
        }
    }
    if seen[0][a[n - 1]] || seen[1][a[n - 1]] {
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
    const PROBLEM_NAME: &str = "b";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

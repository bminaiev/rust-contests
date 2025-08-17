//{"name":"A. MEX на матрице","group":"Codeforces - Codeforces Round 1024 (Div. 1)","url":"https://codeforces.com/contest/2101/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2\n3\n","output":"0 1\n2 3\n8 4 5\n6 0 1\n7 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMEXNaMatritse"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = Array2D::new(0, n, n);
        let start = (n - 1) / 2;
        let mut cur_n = 1;
        let mut sz = 1;
        let mut curx = start as i32;
        let mut cury = start as i32;
        // draw spiral
        while cur_n != n {
            for i in 0..sz {
                a[curx as usize][cury as usize] = cur_n;
                cur_n += 1;
                cury += 1;
            }
            cury -= 1;
            curx += 1;
            for i in 0..sz {
                a[curx as usize][cury as usize] = cur_n;
                cur_n += 1;
                curx += 1;
            }
            curx -= 1;
            sz += 2;
            for i in 0..sz {
                a[curx as usize][cury as usize] = cur_n;
                cur_n += 1;
                cury -= 1;
            }
            cury += 1;
            curx -= 1;
            for i in 0..sz {
                a[curx as usize][cury as usize] = cur_n;
                cur_n += 1;
                curx -= 1;
            }
            curx += 1;
        }
        for i in 0..n {
            out.println(a[i].to_vec());
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_mexna_matritse";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

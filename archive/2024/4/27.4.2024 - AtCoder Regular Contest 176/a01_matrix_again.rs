//{"name":"A - 01 Matrix Again","group":"AtCoder - AtCoder Regular Contest 176","url":"https://atcoder.jp/contests/arc176/tasks/arc176_a","interactive":false,"timeLimit":4000,"tests":[{"input":"4 2\n1 4\n3 2\n","output":"8\n1 2\n1 4\n2 1\n2 4\n3 2\n3 3\n4 1\n4 3\n"},{"input":"3 3\n3 1\n2 3\n1 3\n","output":"9\n1 1\n1 2\n1 3\n2 1\n2 2\n2 3\n3 1\n3 2\n3 3\n"},{"input":"7 3\n1 7\n7 6\n6 1\n","output":"21\n1 6\n2 4\n4 1\n7 3\n3 6\n4 5\n6 1\n1 7\n7 6\n3 5\n2 2\n6 3\n6 7\n5 4\n5 2\n2 5\n5 3\n1 4\n7 1\n4 7\n3 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A01MatrixAgain"}}}

use std::vec;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut pairs = vec![];
    for _ in 0..m {
        pairs.push((input.usize() - 1, input.usize() - 1));
    }
    pairs.sort();
    let mut rnd = Random::new(8778787);
    loop {
        let mut res = pairs.clone();
        let mut cnt_r = vec![m; n];
        let mut cnt_c = vec![m; n];
        for &(r, c) in pairs.iter() {
            cnt_r[r] -= 1;
            cnt_c[c] -= 1;
        }
        let perm = rnd.gen_permutation(n);
        let mut it = 0;
        for r in 0..n {
            if cnt_r[r] > 0 {
                while cnt_c[it] == 0 {
                    it += 1;
                }
                for c_it in it..n {
                    let c = perm[c_it];
                    if cnt_c[c] > 0 && !pairs.contains(&(r, c)) {
                        res.push((r, c));
                        cnt_r[r] -= 1;
                        cnt_c[c] -= 1;
                    }
                    if cnt_r[r] == 0 {
                        break;
                    }
                }
            }
            if cnt_r[r] == 0 {
                break;
            }
        }
        if res.len() == n * m {
            out.println(res.len());
            for (r, c) in res {
                out.println(vec![r + 1, c + 1]);
            }
            break;
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
    const PROBLEM_NAME: &str = "a01_matrix_again";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

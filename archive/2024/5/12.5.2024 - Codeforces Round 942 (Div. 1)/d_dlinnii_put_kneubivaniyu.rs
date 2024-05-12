//{"name":"D. Длинный путь к неубыванию","group":"Codeforces - Codeforces Round 942 (Div. 1)","url":"https://codeforces.com/contest/1967/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n5 8\n1 6 3 7 1\n2 3 5 8 7 1 5 6\n3 3\n1 3 2\n2 1 3\n10 10\n2 8 5 4 8 4 1 5 10 10\n6 7 2 6 3 4 1 1 3 5\n","output":"3\n-1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDlinniiPutKNeubivaniyu"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let nxt = input.vec::<usize>(m).sub_from_all(1);
    const MX: usize = 20;
    let mut go = Array2D::new(0, MX, m);
    let mut mn = Array2D::new(0, MX, m);
    for i in 0..m {
        go[0][i] = nxt[i];
        mn[0][i] = i;
    }
    for lvl in 1..MX {
        for v in 0..m {
            go[lvl][v] = go[lvl - 1][go[lvl - 1][v]];
            let next_min = mn[lvl - 1][v].min(mn[lvl - 1][go[lvl - 1][v]]);
            mn[lvl][v] = next_min;
        }
        // dbg!(lvl, mn[lvl]);
    }
    let max_res = m + 5;
    let res = binary_search_first_true(0..max_res, |len| {
        let mut last = 0;
        let mut cur_mn = mn.clone();

        // dbg!("Check", len);

        for &start_value in a.iter() {
            let mut start_value = start_value;
            let mut best = usize::MAX;
            for bit in (0..MX).rev() {
                if ((1 << bit) & len) != 0 {
                    let test_best = get_min(&mut cur_mn, &go, start_value, bit, last);
                    if test_best < best {
                        best = test_best;
                    }
                    start_value = go[bit][start_value];
                }
            }
            // dbg!(start_value, best);
            if best == usize::MAX {
                return false;
            }
            last = best;
            // dbg!(last);
        }
        true
    });
    if res == max_res {
        out.println(-1);
    } else {
        out.println(res - 1);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_dlinnii_put_kneubivaniyu";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve_slow(s: &[Vec<u8>]) -> Option<Vec<Vec<bool>>> {
    let n = s.len();
    let m = s[0].len();
    let total = n * m;
    let mut ids = Array2D::new(0, n, m);
    let mut need_mask = 0;
    for i in 0..n {
        for j in 0..m {
            ids[i][j] = i * m + j;
            if s[i][j] == b'1' {
                need_mask |= 1 << ids[i][j];
            }
        }
    }
    let mut apply = Array2D::new(0, n, m);
    for i in 0..n {
        for j in 0..m {
            for i2 in 0..n {
                for j2 in 0..m {
                    let d = i.abs_diff(i2) + j.abs_diff(j2);
                    if d <= 1 {
                        apply[i][j] |= 1 << ids[i2][j2];
                    }
                }
            }
        }
    }
    for mask in 0..1 << total {
        let mut xor_mask = 0;
        for i in 0..n {
            for j in 0..m {
                if (mask >> ids[i][j]) & 1 == 1 {
                    xor_mask ^= apply[i][j];
                }
            }
        }
        if xor_mask == need_mask {
            let mut ans = vec![vec![false; m]; n];
            for i in 0..n {
                for j in 0..m {
                    if (mask >> ids[i][j]) & 1 == 1 {
                        ans[i][j] = true;
                    }
                }
            }
            return Some(ans);
        }
    }
    None
}

fn solve123(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    let mut s = vec![];
    for _ in 0..n {
        let ss = input.string();
        s.push(ss);
    }
    if let Some(ans) = solve_slow(&s) {
        out.println("YES");
        for i in 0..n {
            for j in 0..m {
                if ans[i][j] {
                    out.print("1");
                } else {
                    out.print("0");
                }
            }
            out.println("");
        }
    } else {
        out.println("NO");
    }
}

fn solve444(input: &mut Input, out: &mut Output) {
    let n = 3;
    let m = 5;
    for x in 0..n {
        for y in 0..m {
            let mut s = vec![vec![b'0'; m]; n];
            s[x][y] = b'1';
            out.println(vec![x, y]);
            if let Some(ans) = solve_slow(&s) {
                out.println("YES");
                for i in 0..n {
                    for j in 0..m {
                        if ans[i][j] {
                            out.print("1");
                        } else {
                            out.print("0");
                        }
                    }
                    out.println("");
                }
            } else {
                out.println("NO");
            }
            out.flush();
        }
    }
}

fn solve12333(input: &mut Input, out: &mut Output) {
    let n = 5;
    let m = 5;
    let mut apply = Array2D::new(0, n, m);
    let total = n * m;
    let mut cnt = vec![0; 1 << total];
    let mut ids = Array2D::new(0, n, m);
    for i in 0..n {
        for j in 0..m {
            ids[i][j] = i * m + j;
        }
    }
    for i in 0..n {
        for j in 0..m {
            for i2 in 0..n {
                for j2 in 0..m {
                    let d = i.abs_diff(i2) + j.abs_diff(j2);
                    if d <= 1 {
                        apply[i][j] |= 1 << ids[i2][j2];
                    }
                }
            }
            cnt[apply[i][j] as usize] += 1;
        }
    }
    let mut by_cnt = vec![0; 10];
    for mask in 0usize..1 << total {
        by_cnt[cnt[mask]] += 1;
    }
    dbg!(by_cnt[1..]);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}

//{"name":"C - Human Exercise","group":"AtCoder - AtCoder Grand Contest 072","url":"https://atcoder.jp/contests/agc072/tasks/agc072_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n","output":"RRDDRRDD\n"},{"input":"10 869120\n","output":"RDRRRDRDRDRDRDDDRD\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CHumanExercise"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {}

fn stress() {
    let n = 20;
    let mut cnt = Array2D::new(0, n, n);
    let mut times = vec![vec![vec![]; n]; n];
    let iters = 104006;
    let mut res = vec![];
    for i in 0..iters {
        let last = i == iters - 1;
        let mut x = 0;
        let mut y = 0;
        while x + 1 < n && y + 1 < n {
            if cnt[x + 1][y] <= cnt[x][y + 1] {
                x += 1;
                if last {
                    res.push('D');
                }
            } else {
                y += 1;
                if last {
                    res.push('R');
                }
            }
            times[x][y].push(i);
            cnt[x][y] += 1;
        }
        while x + 1 < n {
            x += 1;
            times[x][y].push(i);
            cnt[x][y] += 1;
            if last {
                res.push('D');
            }
        }
        while y + 1 < n {
            y += 1;
            times[x][y].push(i);
            cnt[x][y] += 1;
            if last {
                res.push('R');
            }
        }
    }
    // for i in 0..n {
    //     let mut str = String::new();
    //     for j in 0..n {
    //         str += &format!("{:6} ", cnt[i][j]);
    //     }
    //     dbg!(str);
    // }
    for sum in 1..n {
        let mut str = String::new();
        for i in (1..n).rev() {
            for j in 0..n {
                if i + j == sum {
                    str += &format!("{:1}", cnt[i][j] - cnt[i - 1][j + 1]);
                }
            }
        }
        dbg!(sum, str);
    }
    // for sum in 1..=4 {
    //     dbg!(sum);
    //     let mut str = String::new();
    //     for i in (0..n).rev() {
    //         for j in 0..n {
    //             if i + j == sum {
    //                 str += &format!("{:?} ", times[i][j]);
    //             }
    //         }
    //     }
    //     dbg!(str);
    // }
    let mut res = res.iter().collect::<String>();
    dbg!(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c_human_exercise";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

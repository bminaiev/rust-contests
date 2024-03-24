//{"name":"C - Not Median","group":"AtCoder - AtCoder Regular Contest 173","url":"https://atcoder.jp/contests/arc173/tasks/arc173_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 3 5 4 2\n","output":"3 3 3 5 3\n"},{"input":"3\n2 1 3\n","output":"-1 3 3\n"},{"input":"14\n7 14 6 8 10 2 9 5 4 12 11 3 13 1\n","output":"5 3 3 7 3 3 3 5 3 3 5 3 3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNotMedian"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i32>(n);
    let mut res = vec![-1; n];
    {
        let mut more = 0;
        let mut less = 0;
        let center = a[0];
        for pos in 1..n {
            if a[pos] > center {
                more += 1;
            } else {
                less += 1;
            }
            if (more + less) % 2 == 0 {
                if more != less {
                    res[0] = (pos + 1) as i32;
                    break;
                }
            }
        }
    }
    {
        let mut more = 0;
        let mut less = 0;
        let center = a[n - 1];
        for pos in (0..n - 1).rev() {
            if a[pos] > center {
                more += 1;
            } else {
                less += 1;
            }
            if (more + less) % 2 == 0 {
                if more != less {
                    res[n - 1] = more + less + 1;
                    break;
                }
            }
        }
    }
    for mid in 1..n - 1 {
        let center = a[mid];
        if (a[mid - 1] < center) == (a[mid + 1] < center) {
            res[mid] = 3;
        } else {
            let max_d = (n - mid).max(mid + 1);
            for d in 1..max_d {
                let mut expect_right = a[mid + 1] < center;
                if d % 2 == 0 {
                    expect_right = !expect_right;
                }
                let new_res = (1 + (d + 1) / 2 * 2) as i32;
                if mid + d < n && (a[mid + d] < center) != expect_right {
                    res[mid] = new_res;
                    break;
                }
                if mid >= d && (a[mid - d] < center) == expect_right {
                    res[mid] = new_res;
                    break;
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
    const PROBLEM_NAME: &str = "c_not_median";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

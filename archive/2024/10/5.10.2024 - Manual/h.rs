//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n_lvl = input.usize();
    let cnt_moves = input.usize();
    let n = 1 << n_lvl;
    let mut sum = vec![0; n * 2 - 1];
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut pos = vec![0; n];
    for i in 0..n {
        pos[a[i]] = i;
    }
    let mut res = vec![0; n];
    for value in 0..n {
        let pos = pos[value];
        {
            let mut v = pos + n - 1;
            let mut cur_res = 0;
            let mut cur_size = 1;
            loop {
                v = (v - 1) / 2;
                cur_size *= 2;

                let cnt_less = sum[v];
                let cnt_more = cur_size - cnt_less - 1;

                let max_available_less = value;

                if cnt_more > cnt_moves || max_available_less + 1 < cur_size {
                    break;
                }

                cur_res += 1;
                if cur_size == n {
                    break;
                }
            }
            res[pos] = cur_res;
        }
        {
            let mut v = pos + n - 1;
            loop {
                sum[v] += 1;
                if v == 0 {
                    break;
                }
                v = (v - 1) / 2;
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
    const PROBLEM_NAME: &str = "h";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};

enum Next {
    Ok(Vec<i64>),
    OneOf(Vec<i64>, Vec<i64>),
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<i64>((1 << (n + 1)) - 2);
        let mut starts = vec![];
        for _i in a.len() / 2 - 1..a.len() {
            starts.push(vec![0]);
        }
        const MX: usize = usize::MAX / 2;
        let res = RecursiveFunction3::new(
            |f, vals: Vec<Vec<i64>>, lvl: usize, more_ops: usize| -> usize {
                if lvl == 0 {
                    assert_eq!(vals.len(), 1);
                    0
                } else {
                    let start = (1 << lvl) - 2;
                    let end = (1 << (1 + lvl)) - 2;
                    let extra = &a[start..end];
                    let mut next = vec![];
                    assert_eq!(extra.len(), vals.len());
                    for i in 0..vals.len() / 2 {
                        let mut left = vals[i * 2].clone();
                        let mut right = vals[i * 2 + 1].clone();
                        for x in left.iter_mut() {
                            *x += extra[2 * i];
                        }
                        for x in right.iter_mut() {
                            *x += extra[2 * i + 1];
                        }
                        let mut full = left.clone();
                        full.extend(right.clone());
                        full.sort();
                        let mut ok = true;
                        for w in full.windows(2) {
                            if w[0] == w[1] {
                                ok = false;
                                break;
                            }
                        }
                        if ok {
                            next.push(Next::Ok(full));
                        } else {
                            next.push(Next::OneOf(left, right));
                        }
                    }
                    let mut cnt_bad = 0;
                    for nx in next.iter() {
                        match nx {
                            Next::Ok(_) => {}
                            Next::OneOf(_, _) => {
                                cnt_bad += 1;
                            }
                        }
                    }
                    if cnt_bad > more_ops {
                        return MX;
                    }
                    let mut result = MX;
                    for mask in 0..(1 << cnt_bad) {
                        let mut cur_mask = mask;
                        let mut real_next = vec![];
                        for nx in next.iter() {
                            match nx {
                                Next::Ok(v) => {
                                    real_next.push(v.clone());
                                }
                                Next::OneOf(left, right) => {
                                    if cur_mask & 1 == 0 {
                                        real_next.push(left.clone());
                                    } else {
                                        real_next.push(right.clone());
                                    }
                                    cur_mask >>= 1;
                                }
                            }
                        }
                        let res_child = f.call(real_next, lvl - 1, more_ops - cnt_bad + 1);
                        result = result.min(res_child + cnt_bad);
                    }
                    result
                }
            },
        )
        .call(starts, n, 1);
        if res == MX {
            out.println(-1);
        } else {
            out.println(res);
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
    const PROBLEM_NAME: &str = "i";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let a = input.vec::<usize>(n);
        let b = input.vec::<usize>(m);

        let mut masks = vec![];
        for i in 0..31 {
            masks.push((1 << i) - 1);
        }
        let mut hm = vec![FxHashMap::default(); masks.len()];

        for i in 0..a.len() - 1 {
            let a1 = a[i];
            let a2 = a[i + 1];

            let axor = a1 ^ a2;
            let pow2 = axor + 2;
            if pow2 <= 3 {
                continue;
            }
            let mask = pow2 - 1;

            let need_f00 = pow2 / 2 - 2;
            let need_b1 = (need_f00 ^ a1) & mask;

            if !(axor + 2).is_power_of_two() {
                continue;
            }

            let need_b1_1 = a1 & 1;

            if (need_b1 & 1) != (need_b1_1) {
                continue;
            }

            let mask_idx = masks.binary_search(&mask).unwrap();
            hm[mask_idx]
                .entry(need_b1)
                .and_modify(|e| *e += 1i64)
                .or_insert(1);
        }

        let mut res = 0;
        for j in 0..b.len() - 1 {
            if (b[j] ^ b[j + 1]) != 1 {
                continue;
            }
            for idx in 0..masks.len() {
                let mask = masks[idx];
                let need_b1 = b[j] & mask;
                res += hm[idx].get(&need_b1).unwrap_or(&0);
            }
        }
        out.println(res);
    }
}

fn stress() {
    const X: usize = 75;
    let mut cnt_fails = 0;
    for a1 in 0..X {
        for a2 in 0..X {
            for b1 in 0..X {
                for b2 in 0..X {
                    let f00 = a1 ^ b1;
                    let f01 = a1 ^ b2;
                    let f10 = a2 ^ b1;
                    let f11 = a2 ^ b2;

                    let axor = a1 ^ a2;
                    let bxor = b1 ^ b2;
                    let pow2 = axor + 2;
                    let mask = pow2 - 1;

                    let real_ok = f01 == f00 + 1 && f11 == f10 + 1 && f10 == f00 + 2;

                    let mut my_ok = true;

                    let mut fail_stage = 0;

                    if pow2 <= 3 {
                        my_ok = false;
                    } else {
                        let need_f00 = pow2 / 2 - 2;
                        let need_b1 = (need_f00 ^ a1) & mask;

                        if !(axor + 2).is_power_of_two() {
                            my_ok = false;
                            fail_stage = 1;
                        }
                        if bxor != 1 {
                            my_ok = false;
                            fail_stage = 2;
                        }

                        if (b1 & mask) != need_b1 {
                            my_ok = false;
                            fail_stage = 3;
                        }

                        let need_b1_1 = a1 & 1;

                        if (need_b1 & 1) != (need_b1_1) {
                            my_ok = false;
                            fail_stage = 4;
                        }
                    }

                    if my_ok != real_ok {
                        dbg!(my_ok, real_ok, a1, a2, b1, b2, f00, f01, f10, f11, fail_stage, mask);
                        cnt_fails += 1;
                    }
                }
            }
        }
    }
    dbg!(cnt_fails);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "j";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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

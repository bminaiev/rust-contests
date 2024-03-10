//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::collections::fx_hash_map::FxHashMap;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod7;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {}

type Mod = Mod7;

fn conv_state(state: &[usize]) -> usize {
    let mut res = 0;
    for i in 0..state.len() {
        res = res * 7 + state[i];
    }
    res
}

fn stress() {
    let mult = [1, 2, 3, 4, 5, 6];
    // let mut cur_mul = [1, 1, 1, 1, 1, 1];
    let mut state = [0, 0, 0, 0, 0, 0];
    let mut hm = FxHashMap::default();
    hm.insert(state, Mod::ONE);
    for i in 0..10_000 {
        if i % 100 == 0 {
            dbg!(i, hm.len());
        }
        let mut next_hm = FxHashMap::default();
        for (k, v) in hm.iter() {
            for bit in 0..2 {
                let mut next_state = *k;
                for j in 0..next_state.len() {
                    next_state[j] *= mult[j];
                    next_state[j] += bit;
                    // next_state[j] += bit * cur_mul[j];
                    next_state[j] %= 7;
                }
                *next_hm.entry(next_state).or_default() += *v;
            }
        }
        {
            let mut non0 = 0;
            for k in hm.into_keys() {
                if k.iter().all(|x| *x != 0) {
                    non0 += 1;
                }
            }
            dbg!(non0);
        }
        hm = next_hm;
        // for j in 0..cur_mul.len() {
        //     cur_mul[j] *= mult[j];
        //     cur_mul[j] %= 7;
        // }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

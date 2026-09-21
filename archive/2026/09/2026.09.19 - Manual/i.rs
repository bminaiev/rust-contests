use std::time::Instant;
use std::{collections::VecDeque, sync::atomic::AtomicI32};

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

// false -- smaller first
fn play(
    a: &VecDeque<usize>,
    b: &VecDeque<usize>,
    seed: u64,
    limit: usize,
    res: &mut Vec<bool>,
) -> usize {
    let mut a = a.clone();
    let mut b = b.clone();

    let mut rnd = Random::new(seed);
    for _ in 0..10 {
        rnd.gen_double();
    }
    let ra = rnd.gen_double() * 0.001;
    let rb = rnd.gen_double() * 0.001;
    let mut res_iter = 0;
    while !a.is_empty() && !b.is_empty() {
        if res_iter > limit {
            break;
        }
        let af = a.pop_front().unwrap();
        let bf = b.pop_front().unwrap();
        let res_val = if res_iter < res.len() {
            res_iter += 1;
            res[res_iter - 1]
        } else {
            let nres = if af > bf {
                rnd.gen_double() >= ra
            } else {
                rnd.gen_double() >= rb
            };
            res.push(nres);
            res_iter += 1;
            nres
        };
        let mut vals_to_push = [af.min(bf), af.max(bf)];
        if res_val {
            vals_to_push.swap(0, 1);
        }
        if af > bf {
            a.push_back(vals_to_push[0]);
            a.push_back(vals_to_push[1]);
        } else {
            b.push_back(vals_to_push[0]);
            b.push_back(vals_to_push[1]);
        }
    }
    res.truncate(res_iter);
    res_iter
}

fn convert_to_str(a: &[bool]) -> String {
    let mut a = a.to_vec();
    while a.len() % 4 != 0 {
        a.insert(0, false);
    }
    let mut res = vec![];
    for i in 0..(a.len() / 4) {
        let mut val = 0;
        for j in 0..4 {
            val = val * 2 + (a[i * 4 + j] as u8);
        }
        let c = if val < 10 {
            b'0' + val
        } else {
            b'A' + (val - 10)
        };
        res.push(c);
    }
    String::from_utf8(res).unwrap()
}

fn optimize(
    a: &VecDeque<usize>,
    b: &VecDeque<usize>,
    answer: &mut Vec<bool>,
    new_limit: usize,
) -> bool {
    if answer.len() < new_limit {
        return true;
    }
    let mut rnd = Random::new(123213);
    let mut it = 10_000;
    while it > 0 {
        it -= 1;
        let pos = rnd.gen_range(0..answer.len());
        answer[pos] = !answer[pos];
        let prev_len = answer.len();
        let new_len = play(a, b, it, prev_len + 100, answer);
        if new_len > prev_len {
            answer[pos] = !answer[pos];
            let prev_len2 = play(a, b, it, prev_len + 10, answer);
            assert_eq!(prev_len, prev_len2);
        } else if new_len < prev_len {
            it = 10_000;
            if new_len < new_limit {
                return true;
            }
        }
    }
    false
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    let mut mx = 0;
    let start = Instant::now();
    let mut inputs = vec![];
    for tt in 0..tc {
        let n1 = input.usize();
        let n2 = input.usize();
        let a: VecDeque<usize> = input.vec::<usize>(n1).into_iter().collect();
        let b: VecDeque<usize> = input.vec::<usize>(n2).into_iter().collect();
        inputs.push((a, b));
    }
    let solved = AtomicI32::default();
    let outputs: Vec<_> = inputs[..100]
        .into_par_iter()
        .map(|(a, b)| {
            const LIMIT: usize = 25_000;
            const SECOND_LIMIT: usize = 20_000;
            for seed in 10.. {
                let mut res = vec![];
                let xx = play(a, b, seed, LIMIT, &mut res);
                if xx < LIMIT && optimize(a, b, &mut res, SECOND_LIMIT) {
                    let solved = solved.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    dbg!(seed, solved, tc, xx, res.len());
                    return (res.len(), convert_to_str(&res));
                }
            }
            unreachable!()
        })
        .collect();
    for (len, s) in outputs {
        out.println(format!("{len} {s}"));
    }
    dbg!(start.elapsed());
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "i";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    run_dragon(solve, "I", 2..3);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}

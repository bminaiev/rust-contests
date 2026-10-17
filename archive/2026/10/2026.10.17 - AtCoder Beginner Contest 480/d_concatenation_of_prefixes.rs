#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::pref_sum::PrefSum;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let q = input.usize();
    let a = input.vec::<i64>(n);
    let pref_a = a.pref_sum();
    let mut pref_pref = vec![0];
    for len in 1..=n {
        let prev = pref_pref[len - 1];
        let new_sum = prev + pref_a[len];
        pref_pref.push(new_sum);
    }
    let get_pref = |len: usize| -> i64 {
        let cnt_full = binary_search_first_true(0..n + 1, |cnt| {
            let tot_cnt = cnt * (cnt + 1) / 2;
            tot_cnt > len
        }) - 1;
        let extra = len - (cnt_full * (cnt_full + 1) / 2);
        pref_pref[cnt_full] + pref_a[extra]
    };
    for _ in 0..q {
        let l = input.usize() - 1;
        let r = input.usize();
        out.println(get_pref(r) - get_pref(l));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "d_concatenation_of_prefixes";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}

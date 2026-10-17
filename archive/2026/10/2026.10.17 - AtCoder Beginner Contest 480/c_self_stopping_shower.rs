#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let ok = input.i64();
    let max_t = input.i64();
    let mut moments = vec![];
    for _ in 0..n {
        moments.push(input.i64());
    }
    moments.sort();
    while moments.len() > 0 && moments[moments.len() - 1] >= max_t {
        moments.pop();
    }
    let mut res = 0;
    let mut add = |len: i64| {
        if len <= 0 {
            return;
        }
        res += (len + ok - 1) / ok;
    };
    if moments.is_empty() {
        add(max_t);
    } else {
        add(moments[0]);
        add(max_t - moments[moments.len() - 1] - ok);
        for w in moments.windows(2) {
            add(w[1] - w[0] - ok);
        }
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_self_stopping_shower";
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

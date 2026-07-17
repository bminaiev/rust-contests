#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::seg_trees::fenwick::Fenwick;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let d = input.usize();
        let a = input.vec::<i64>(n);
        let mut sorted = vec![];
        for i in 0..n {
            sorted.push((a[i], i));
        }
        sorted.sort();
        sorted.reverse();
        let mut res = 0;
        let mut used = Fenwick::<i64>::new(3 * n);
        let mut used_sum = Fenwick::<i64>::new(3 * n);
        // let mut used_bool = vec![false; n];

        for i in 0..n {
            for p in [i, i + n, i + 2 * n] {
                used_sum.add(p, -a[i]);
                used.add(p, 1);
            }
        }

        for (val, pos) in sorted {
            let from = pos + n - d;
            let to = pos + n + d + 1;
            let cost = used_sum.get_range_sum(from..to) + used.get_range_sum(from..to) * val;
            // dbg!(val, pos, cost);
            if cost > 0 {
                res += cost;
                // for p in [pos, pos + n, pos + 2 * n] {
                //     used.add(p, -1);
                //     used_sum.add(p, -val);
                // }
                // used_bool[pos] = true;
            }
        }
        // dbg!(res);
        // dbg!(used_bool);
        // for i in 0..n {
        //     let from = i + n - d;
        //     let to = i + n + d + 1;
        //     let mut used_cnt = used.get_range_sum(from..to);
        //     if used_bool[i] {
        //         used_cnt -= 1;
        //     }
        //     let not_used = 2 * d as i64 - used_cnt;
        //     dbg!(i, used_cnt, not_used);
        //     if used_bool[i] {
        //         res += not_used * a[i];
        //     } else {
        //         res -= used_cnt * a[i];
        //     }
        // }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b_";
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

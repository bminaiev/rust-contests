#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

fn solve_case(a: &[usize]) -> Mod {
    let n = a.len() + 1;
    for &x in a {
        if x == n {
            return Mod::ZERO;
        }
    }
    let mut mid = n;
    for i in 0..n - 1 {
        if a[i] == n - 1 {
            mid = i;
            break;
        }
    }
    if mid == n {
        return Mod::ZERO;
    }
    let mut on_the_left = vec![false; n];
    let mut on_the_right = vec![false; n];
    let mut cnt = vec![0; n];
    for i in 0..n - 1 {
        if i < mid {
            on_the_left[a[i]] = true;
        } else if i > mid {
            on_the_right[a[i]] = true;
        }
        cnt[a[i]] += 1;
    }
    for i in 0..n {
        if on_the_left[i] && on_the_right[i] {
            return Mod::ZERO;
        }
    }
    for i in 1..mid {
        if a[i] < a[i - 1] {
            return Mod::ZERO;
        }
    }
    for i in mid..n - 2 {
        if a[i] < a[i + 1] {
            return Mod::ZERO;
        }
    }
    let mut ways = Mod::ONE;
    let mut free = 0;
    for value in 1..n {
        if cnt[value] == 0 {
            free += 1;
        } else {
            let extra = cnt[value] - 1;
            if extra > free {
                return Mod::ZERO;
            }
            for _ in 0..extra {
                ways *= Mod::new(free);
                free -= 1;
            }
        }
    }
    assert!(free == 0);
    ways
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = input.vec::<usize>(n - 1);
        let mut res = solve_case(&a);
        a.reverse();
        res += solve_case(&a);
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
    const PROBLEM_NAME: &str = "b_permutation_cuts";
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

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::fenwick::Fenwick;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut res = Mod::ZERO;
    let pow2 = Mod::new(2).pown(n - 2);
    let mut fenw = Fenwick::<usize>::new(n);
    let mut cnt_right_smaller = vec![0; n];
    for i in (0..n).rev() {
        cnt_right_smaller[i] = fenw.get_sum(a[i]);
        fenw.add(a[i], 1);
    }
    fenw.clear();
    for i in 0..n {
        let cnt_left_bigger = fenw.get_suffix_sum(a[i]);
        let cnt_invs = cnt_left_bigger + cnt_right_smaller[i];
        res += Mod::new(cnt_left_bigger) * pow2;

        let extra_ways = Mod::new(2).pown(n - 1 - cnt_invs);
        res += extra_ways * f(cnt_left_bigger, cnt_right_smaller[i]);
        fenw.add(a[i], 1);
    }
    out.println(res);
}

fn f(n: usize, m: usize) -> Mod {
    let nn = n + m;
    let tmp_res = Mod::new(2).pown(nn - 1) * Mod::new(nn);
    tmp_res - g(n, m) * Mod::new(2)
}

fn g(n: usize, m: usize) -> Mod {
    let mut res = Mod::ZERO;
    for left_mask in 0usize..(1 << n) {
        for right_mask in 0usize..(1 << m) {
            let l_cnt = left_mask.count_ones();
            let r_cnt = right_mask.count_ones();
            res += Mod::new(l_cnt.min(r_cnt));
        }
    }
    res
}

fn stress() {
    const N: usize = 8;
    let mut a = Array2D::new(Mod::ZERO, N, N);
    for n in 1..N {
        for m in 1..N {
            a[n][m] = g(n, m);
        }
        dbg!(a[n]);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_partially_sort";
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

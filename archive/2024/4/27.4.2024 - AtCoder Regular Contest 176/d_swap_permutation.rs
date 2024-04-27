//{"name":"D - Swap Permutation","group":"AtCoder - AtCoder Regular Contest 176","url":"https://atcoder.jp/contests/arc176/tasks/arc176_d","interactive":false,"timeLimit":4000,"tests":[{"input":"3 1\n1 3 2\n","output":"8\n"},{"input":"2 5\n2 1\n","output":"1\n"},{"input":"5 2\n3 5 1 4 2\n","output":"833\n"},{"input":"20 24\n14 1 20 6 11 3 19 2 7 10 9 18 13 12 17 8 15 5 4 16\n","output":"203984325\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSwapPermutation"}}}

use std::ops::Range;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let p = input.vec::<i32>(n).sub_from_all(1);
    let n_mod = Mod::new(n as i32);
    let n_left = n_mod - Mod::TWO;
    let mut g = Array2D::new(Mod::ZERO, 4, 4);
    for i in 0..4 {
        g[i][i] = Mod::ONE + n_left * (n_left - Mod::ONE) / Mod::TWO;
    }
    g[0][1] = n_left;
    g[0][2] = n_left;
    for i in 1..=2 {
        g[i][0] = Mod::ONE;
        g[i][i] += n_left - Mod::ONE;
        g[i][3 - i] = Mod::ONE;
        g[i][3] = n_left - Mod::ONE;
    }
    for i in 1..=2 {
        g[3][i] = Mod::TWO;
    }
    // dbg!(g[3]);
    g[3][3] += Mod::TWO * (n_left - Mod::TWO);
    let expected_sum = n_mod * (n_mod - Mod::ONE) / Mod::TWO;
    // dbg!(n);
    if n != 2 {
        for i in 0..4 {
            // dbg!(g[i]);
            let mut cur_sum = Mod::ZERO;
            for j in 0..4 {
                cur_sum += g[i][j];
            }
            assert_eq!(cur_sum, expected_sum);
        }
    }
    let mut cur = vec![Mod::ZERO; 4];
    cur[0] = Mod::ONE;
    for _ in 0..m {
        let mut next = vec![Mod::ZERO; 4];
        for i in 0..4 {
            for j in 0..4 {
                next[j] += cur[i] * g[i][j];
            }
        }
        cur = next;
    }
    let mut res = Mod::ZERO;
    let mut sum = Mod::ZERO;
    let mut cnt_edges = Mod::ZERO;
    for i in 1..n as i32 {
        sum += Mod::new(i) * Mod::new(i + 1) / Mod::TWO;
        cnt_edges += Mod::new(i);
    }
    // dbg!(cur);
    // dbg!(get_av_dist(2, 0, 3));
    // dbg!(sum);
    for w in p.windows(2) {
        let cur_diff = (w[0] - w[1]).abs();
        res += Mod::new(cur_diff) * cur[0];
        res += get_av_dist(w[0], w[1], n as i32) * cur[1];
        res += get_av_dist(w[1], w[0], n as i32) * cur[2];
        // dbg!(w[0], w[1], get_av_dist(w[0], w[1], n as i32));
        // dbg!(w[1], w[0], get_av_dist(w[1], w[0], n as i32));
        {
            let min = w[0].min(w[1]);
            let max = w[0].max(w[1]);
            let mut sum_dist_here = sum;
            sum_dist_here -= get_av_dist(min, max, n as i32) * Mod::new(n - 2);
            sum_dist_here -= get_av_dist(max, min, n as i32) * Mod::new(n - 2);
            sum_dist_here -= Mod::new(max - min);
            let tot_edges = cnt_edges - Mod::new(n - 2) * Mod::TWO - Mod::ONE;
            if tot_edges != Mod::ZERO {
                // dbg!(sum_dist_here);
                let av_dist_here = sum_dist_here / tot_edges;
                // dbg!(av_dist_here);
                res += av_dist_here * cur[3];
            }
        }
    }
    out.println(res);
}

fn get_av_dist(x: i32, y: i32, n: i32) -> Mod {
    if n == 2 {
        return Mod::ZERO;
    }
    let mut res = Mod::ZERO;
    if x < y {
        res = Mod::new(x) * Mod::new(x) - sum(1..x);
        res += sum(x + 1..y) + sum(y + 1..n);
        res -= Mod::new(x) * Mod::new(n - x - 2);
    } else {
        res = sum(x + 1..n) - Mod::new(x) * Mod::new(n - x - 1);
        res += Mod::new(x) * Mod::new(x - 1) - sum(1..y) - sum(y + 1..x);
    }
    res / Mod::new(n - 2)
}

fn sum(r: Range<i32>) -> Mod {
    let from = Mod::new(r.start);
    let to = Mod::new(r.end - 1);
    (to + from) * (to - from + Mod::ONE) / Mod::TWO
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_swap_permutation";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "4");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

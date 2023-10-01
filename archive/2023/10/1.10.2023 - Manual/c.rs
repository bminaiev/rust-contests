//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use std::cmp::min;
use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

fn solve_g(g: &Array2D<bool>) -> Mod {
    let n = g.len();
    let mut deg: Vec<usize> = gen_vec(n, |id| (0..n).map(|to| if g[id][to] { 1 } else { 0 }).sum());
    let mut seen = vec![false; n];
    let mut res = Mod::ZERO;
    for _ in 0..n {
        let v = (0..n)
            .filter(|&v| !seen[v])
            .min_by_key(|&v| deg[v])
            .unwrap();
        seen[v] = true;
        let mut neigh = vec![];
        for to in 0..n {
            if g[v][to] && !seen[to] {
                deg[to] -= 1;
                neigh.push(to);
            }
        }
        assert!(neigh.len() <= 44);
        let first_part = min((neigh.len() + 3) / 2, neigh.len());
        let (first, second) = neigh.split_at(first_part);
        let mut first_mask = vec![0; neigh.len()];
        let mut second_mask = vec![0; neigh.len()];
        for i in 0..neigh.len() {
            let v = neigh[i];
            for j in 0..first.len() {
                if g[v][first[j]] {
                    first_mask[i] |= 1 << j;
                }
            }
            for j in 0..second.len() {
                if g[v][second[j]] {
                    second_mask[i] |= 1 << j;
                }
            }
        }
        let mut dp = vec![Mod::ZERO; 1 << second.len()];
        let mut ok_first_mask = vec![(1 << first.len()) - 1; 1 << first.len()];
        let mut ok_second_mask = vec![(1 << second.len()) - 1; 1 << first.len()];
        for mask in 0usize..(1 << first.len()) {
            if mask != 0 {
                let first_bit = mask.trailing_zeros() as usize;
                ok_first_mask[mask] =
                    ok_first_mask[mask ^ (1 << first_bit)] & first_mask[first_bit];
                ok_second_mask[mask] =
                    ok_second_mask[mask ^ (1 << first_bit)] & second_mask[first_bit];
            }
            if mask & ok_first_mask[mask] != mask {
                continue;
            }
            dp[ok_second_mask[mask]] += Mod::ONE;
        }
        for i in 0..second.len() {
            for mask in 0..(1 << second.len()) {
                if (1 << i) & mask != 0 {
                    let dp_mask = dp[mask];
                    dp[mask ^ (1 << i)] += dp_mask;
                }
            }
        }
        for mask in 0..(1 << second.len()) {
            let mut ok_second_mask = (1 << second.len()) - 1;
            for i in 0..second.len() {
                if (1 << i) & mask != 0 {
                    ok_second_mask &= second_mask[i + first.len()];
                }
            }
            if mask & ok_second_mask != mask {
                continue;
            }
            res += dp[mask];
        }
    }
    // dbg!(res);
    res
}

fn stress() {
    for it in 1.. {
        let start = Instant::now();
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = 45; //rnd.gen(1..2);
        dbg!(n * (n - 1) / 2);
        let mut m = rnd.gen(0..n * (n - 1) / 2 + 1);
        if m > 1000 {
            m = 1000;
        }
        let mut g = Array2D::new_f(n, n, |i, j| true);
        for _ in 0..m {
            let fr = rnd.gen(0..n);
            let to = rnd.gen(0..n);
            g[fr][to] = true;
            g[to][fr] = true;
        }
        let res = solve_g(&g);
        dbg!(start.elapsed());
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut g = Array2D::new_f(n, n, |i, j| i == j);
    let m = input.usize();
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr][to] = true;
        g[to][fr] = true;
    }
    let res = solve_g(&g);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::chinease_remainder::chinease_remainder;
use algo_lib::math::gcd::lcm;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable5, RecursiveFunction5};
use algo_lib::misc::vec_apply_delta::ApplyDelta;

fn solve_case(parent: &[usize], len_up: &[i128], queries: &[i128]) -> Vec<usize> {
    let n = parent.len();
    // dbg!(n, parent, len_up, queries);
    let mut g = vec![vec![]; n];
    for i in 1..n {
        g[parent[i]].push(i);
    }
    let q = queries.len();
    let mut res = vec![n + 1; q];
    RecursiveFunction5::new(
        |f, v: usize, cur_len: i128, k: i128, b: i128, cur_req_index: Vec<usize>| {
            if cur_req_index.is_empty() {
                return;
            }
            if g[v].is_empty() {
                for id in cur_req_index {
                    res[id] = v + 1;
                }
                return;
            }
            let sz = g[v].len();
            if k > 2_000_000_000_000_000_000_i128 {
                let req = queries[cur_req_index[0]];
                let to = g[v][((req + cur_len) % sz as i128) as usize];
                f.call(to, cur_len + len_up[to], k, b, cur_req_index);
                return;
            }
            let nk = lcm(sz as i128, k);
            if k == nk {
                let to = g[v][((cur_len + b) % sz as i128) as usize];
                f.call(to, cur_len + len_up[to], k, b, cur_req_index);
                return;
            }
            let mut children = vec![vec![]; sz];
            for id in cur_req_index {
                let time = ((queries[id] + cur_len) % sz as i128) as usize;
                children[time].push(id);
            }
            for (i, child_queries) in children.into_iter().enumerate() {
                if child_queries.is_empty() {
                    continue;
                }
                let to = g[v][i];
                let sz_mod =
                    (((i as i128 - cur_len) % sz as i128) + sz as i128) % sz as i128;
                let nb = chinease_remainder(&[sz_mod, b], &[sz as i128, k]).unwrap();
                f.call(to, cur_len + len_up[to], nk, nb, child_queries);
            }
        },
    )
    .call(0, 0, 1, 0, (0..q).collect());
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let q = input.usize();
        let mut parent = input.vec::<usize>(n - 1).sub_from_all(1);
        parent.insert(0, 0);
        let mut len_up = input.vec::<i128>(n - 1);
        len_up.insert(0, 0);
        let queries = input.vec::<i128>(q);
        let res = solve_case(&parent, &len_up, &queries);
        out.println(res);
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..100000);
        let q = rnd.gen_range(1..100000);
        let mut parent = vec![0; n];
        for i in 1..n {
            parent[i] = rnd.gen_range(0..i.min(10));
        }
        let mut len_up = vec![0; n];
        for i in 1..n {
            len_up[i] = rnd.gen_range(0..1000);
        }
        let queries = (0..q).map(|_| rnd.gen_range(0..1_0)).collect::<Vec<_>>();
        let res = solve_case(&parent, &len_up, &queries);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
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

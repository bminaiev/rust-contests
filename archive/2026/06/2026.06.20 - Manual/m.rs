#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::trees::binary_lifting::BinaryLifting;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut g = vec![vec![]; n];
        let mut by_color = vec![vec![]; n];
        for _ in 0..(n - 1) {
            let fr = input.usize() - 1;
            let to = input.usize() - 1;
            let color = input.usize();
            g[fr].push(to);
            g[to].push(fr);
            by_color[color].push((fr, to));
        }
        let binary = BinaryLifting::new(&g, 0);
        let mut in_order = vec![];
        let mut out_order = vec![];

        RecursiveFunction2::new(|f, v: usize, p: usize| {
            in_order.push(v);
            for &u in &g[v] {
                if u == p {
                    continue;
                }
                f.call(u, v);
            }
            out_order.push(v);
        })
        .call(0, 0);

        let mut in_order_pos = vec![0; n];
        let mut out_order_pos = vec![0; n];
        for i in 0..n {
            in_order_pos[in_order[i]] = i;
            out_order_pos[out_order[i]] = i;
        }

        let in_subtree = |root: usize, v: usize| {
            in_order_pos[root] <= in_order_pos[v] && out_order_pos[v] <= out_order_pos[root]
        };

        let is_on_path = |from: usize, to: usize, v: usize| {
            let from_in_subtree = in_subtree(v, from);
            let to_in_subtree = in_subtree(v, to);
            if !from_in_subtree && !to_in_subtree {
                return false;
            }
            if from_in_subtree && to_in_subtree {
                let lca = binary.lca(from, to);
                return lca == v;
            }
            true
        };

        let mut segs = by_color[0].clone();
        let mut res = n;
        if segs.is_empty() {
            res = 0;
        } else {
            for color in 1..n {
                let mut new_segs = vec![];
                for &(u, v) in segs.iter() {
                    let mut already_good = false;
                    for &(a, b) in by_color[color].iter() {
                        if is_on_path(u, v, a) && is_on_path(u, v, b) {
                            already_good = true;
                            break;
                        }
                    }
                    if already_good {
                        new_segs.push((u, v));
                    } else {
                        for &(a, b) in by_color[color].iter() {
                            let mut all_vertices = [u, v, a, b];
                            all_vertices.sort_by_key(|&v| in_order_pos[v]);
                            let start = all_vertices[3];
                            all_vertices.sort_by_key(|&v| out_order_pos[v]);
                            let end = all_vertices[0];
                            // dbg!(a, b, all_vertices, start, end);
                            let mut ok = true;
                            for x in all_vertices {
                                if x == start || x == end || is_on_path(start, end, x) {
                                    continue;
                                }
                                ok = false;
                                break;
                            }
                            if ok {
                                new_segs.push((start, end));
                            } else {
                                all_vertices.sort_by_key(|&v| in_order_pos[v]);
                                let start = all_vertices[0];
                                let end = all_vertices[3];
                                let mut ok = true;
                                for x in all_vertices {
                                    if x == start || x == end || is_on_path(start, end, x) {
                                        continue;
                                    }
                                    ok = false;
                                    break;
                                }
                                if ok {
                                    new_segs.push((start, end));
                                }
                            }
                        }
                    }
                }
                // dbg!(color, &new_segs);
                new_segs.sort();
                new_segs.dedup();
                segs = new_segs;
                if segs.is_empty() {
                    res = color;
                    break;
                }
                segs.sort_by_cached_key(|&(fr, to)| {
                    let lca = binary.lca(fr, to);
                    binary.height(fr) + binary.height(to) - 2 * binary.height(lca)
                });
                segs.truncate(700);
            }
        }
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
    const PROBLEM_NAME: &str = "m";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
